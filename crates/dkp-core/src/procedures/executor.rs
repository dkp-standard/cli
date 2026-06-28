use std::sync::OnceLock;
use std::{
    io::Write,
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::{
    p1::{self, WasiP1Ctx},
    p2::pipe::{MemoryInputPipe, MemoryOutputPipe},
    WasiCtxBuilder,
};

use crate::{error::DkpError, DkpResult, Pack};

use super::schema::{EntryPoint, ProcedureDef};

/// Shared wasmtime Engine with epoch interruption enabled.
/// One engine is reused across all invocations; each call gets its own Store.
static ENGINE: OnceLock<Engine> = OnceLock::new();

fn engine() -> &'static Engine {
    ENGINE.get_or_init(|| {
        let mut config = wasmtime::Config::new();
        config.epoch_interruption(true);
        let engine = Engine::new(&config).expect("failed to create wasmtime Engine");

        // Increment epoch every 1 ms so wall-clock deadlines map to epoch
        // units with ~1 ms resolution.
        let engine_clone = engine.clone();
        std::thread::Builder::new()
            .name("dkp-wasm-epoch".into())
            .spawn(move || loop {
                std::thread::sleep(Duration::from_millis(1));
                engine_clone.increment_epoch();
            })
            .expect("failed to spawn epoch thread");

        engine
    })
}

/// Options for a single procedure invocation.
pub struct RunOptions {
    /// JSON value to serialize and send to the procedure's stdin.
    pub input: serde_json::Value,
    /// Wall-clock timeout in milliseconds.
    /// Falls back to `manifest.procedure_capabilities.max_runtime_ms`, then 5000 ms.
    pub timeout_ms: Option<u64>,
    /// Allow running non-WASM procedures from unsigned bundles (prints warning; default: false).
    pub allow_unsigned: bool,
}

/// Invoke a procedure and return its parsed JSON output.
/// Dispatches to WASM or subprocess execution based on what's available.
pub fn run(pack: &Pack, def: &ProcedureDef, opts: RunOptions) -> DkpResult<serde_json::Value> {
    match (&def.wasm_path, &def.entry_point) {
        (Some(wasm_path), _) => run_wasm(pack, def, wasm_path, opts),
        (None, Some(ep)) => run_subprocess(pack, def, ep, opts),
        (None, None) => Err(DkpError::ProcedureNoExecutable { id: def.id.clone() }),
    }
}

fn resolve_timeout(opts_ms: Option<u64>, pack: &Pack) -> u64 {
    opts_ms
        .or_else(|| {
            pack.manifest
                .procedure_capabilities
                .as_ref()
                .and_then(|c| c.max_runtime_ms)
        })
        .unwrap_or(5_000)
}

fn run_wasm(
    pack: &Pack,
    def: &ProcedureDef,
    wasm_path: &std::path::Path,
    opts: RunOptions,
) -> DkpResult<serde_json::Value> {
    // Spec §9.12: warn (not block) for unsigned bundles
    if !pack.has_bundle_sig() {
        eprintln!(
            "warning: executing procedure '{}' from unsigned bundle '{}'; \
             consider running `dkp sign` before distribution",
            def.id, pack.manifest.name
        );
    }

    let timeout_ms = resolve_timeout(opts.timeout_ms, pack);

    let input_bytes = serde_json::to_vec(&opts.input).map_err(|e| DkpError::AssetParse {
        asset: def.id.clone(),
        source: e,
    })?;

    let eng = engine();
    let module = Module::from_file(eng, wasm_path).map_err(|e| DkpError::ProcedureTrap {
        id: def.id.clone(),
        message: e.to_string(),
    })?;

    let stdout_pipe = MemoryOutputPipe::new(4 * 1024 * 1024); // 4 MiB cap

    let wasi_ctx = WasiCtxBuilder::new()
        .stdin(MemoryInputPipe::new(input_bytes))
        .stdout(stdout_pipe.clone())
        .build_p1();

    let mut linker: Linker<WasiP1Ctx> = Linker::new(eng);
    p1::add_to_linker_sync(&mut linker, |t| t).map_err(|e| DkpError::ProcedureTrap {
        id: def.id.clone(),
        message: e.to_string(),
    })?;

    let mut store = Store::new(eng, wasi_ctx);
    // Each epoch tick ≈ 1 ms; deadline of timeout_ms ticks enforces wall-clock limit.
    store.set_epoch_deadline(timeout_ms);

    let instance =
        linker
            .instantiate(&mut store, &module)
            .map_err(|e| DkpError::ProcedureTrap {
                id: def.id.clone(),
                message: e.to_string(),
            })?;

    let func = instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .map_err(|e| DkpError::ProcedureTrap {
            id: def.id.clone(),
            message: e.to_string(),
        })?;

    match func.call(&mut store, ()) {
        Ok(()) => {}
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("epoch") || msg.contains("interrupt") || msg.contains("timeout") {
                return Err(DkpError::ProcedureTimeout {
                    id: def.id.clone(),
                    limit_ms: timeout_ms,
                });
            }
            return Err(DkpError::ProcedureTrap {
                id: def.id.clone(),
                message: msg,
            });
        }
    }

    let output_bytes = stdout_pipe.contents();

    serde_json::from_slice(&output_bytes).map_err(|e| DkpError::ProcedureInvalidOutput {
        id: def.id.clone(),
        reason: e.to_string(),
    })
}

fn run_subprocess(
    pack: &Pack,
    def: &ProcedureDef,
    ep: &EntryPoint,
    opts: RunOptions,
) -> DkpResult<serde_json::Value> {
    // Spec §9.12: SHOULD refuse non-WASM procs from unsigned bundles
    if !pack.has_bundle_sig() {
        if opts.allow_unsigned {
            eprintln!(
                "warning: executing non-WASM procedure '{}' from unsigned bundle '{}' — \
                 this procedure has full host access (filesystem, network). \
                 Sign this bundle with `dkp sign` before distribution.",
                def.id, pack.manifest.name
            );
        } else {
            return Err(DkpError::ProcedureUnsignedSubprocess { id: def.id.clone() });
        }
    }

    let timeout_ms = resolve_timeout(opts.timeout_ms, pack);
    let proc_dir = pack.procedures_dir();

    // Split "python3 my_proc.py" → ["python3", "my_proc.py"]
    let argv: Vec<&str> = ep.command.split_whitespace().collect();
    let (program, args) = argv.split_first().ok_or_else(|| DkpError::ProcedureTrap {
        id: def.id.clone(),
        message: "entry_point.command is empty".into(),
    })?;

    let input_bytes = serde_json::to_vec(&opts.input).map_err(|e| DkpError::AssetParse {
        asset: def.id.clone(),
        source: e,
    })?;

    let mut child = Command::new(program)
        .args(args)
        .current_dir(&proc_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| DkpError::ProcedureTrap {
            id: def.id.clone(),
            message: format!("failed to spawn '{}': {e}", ep.command),
        })?;

    // Write input JSON to stdin, then close it so the child sees EOF
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(&input_bytes)
            .map_err(|e| DkpError::ProcedureTrap {
                id: def.id.clone(),
                message: format!("failed to write stdin: {e}"),
            })?;
    }

    // Watchdog: kill child after timeout_ms if still running.
    // Shared via Arc<Mutex<Option<Child>>>: watchdog calls kill(), main thread takes ownership.
    let child_shared: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(Some(child)));
    let watchdog_shared = Arc::clone(&child_shared);
    let killed = Arc::new(AtomicBool::new(false));
    let killed_flag = Arc::clone(&killed);
    let id_for_watchdog = def.id.clone();
    std::thread::Builder::new()
        .name(format!("dkp-proc-watchdog-{}", def.id))
        .spawn(move || {
            std::thread::sleep(Duration::from_millis(timeout_ms));
            if let Ok(mut guard) = watchdog_shared.lock() {
                if let Some(ref mut c) = *guard {
                    let _ = c.kill();
                    killed_flag.store(true, Ordering::SeqCst);
                    eprintln!(
                        "dkp: procedure '{}' killed after {}ms timeout",
                        id_for_watchdog, timeout_ms
                    );
                }
            }
        })
        .map_err(|e| DkpError::ProcedureTrap {
            id: def.id.clone(),
            message: format!("failed to spawn watchdog thread: {e}"),
        })?;

    let child = child_shared
        .lock()
        .map_err(|_| DkpError::ProcedureTrap {
            id: def.id.clone(),
            message: "child lock poisoned".into(),
        })?
        .take()
        .ok_or_else(|| DkpError::ProcedureTrap {
            id: def.id.clone(),
            message: "child already taken".into(),
        })?;

    let output = child
        .wait_with_output()
        .map_err(|e| DkpError::ProcedureTrap {
            id: def.id.clone(),
            message: format!("failed to wait for child: {e}"),
        })?;

    if !output.status.success() {
        if killed.load(Ordering::SeqCst) {
            return Err(DkpError::ProcedureTimeout {
                id: def.id.clone(),
                limit_ms: timeout_ms,
            });
        }
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(DkpError::ProcedureTrap {
            id: def.id.clone(),
            message: stderr,
        });
    }

    serde_json::from_slice(&output.stdout).map_err(|e| DkpError::ProcedureInvalidOutput {
        id: def.id.clone(),
        reason: e.to_string(),
    })
}
