use anyhow::Result;
use clap::Args;
use flate2::{write::GzEncoder, Compression};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};
use zip::{write::SimpleFileOptions, ZipWriter};

use dkp_core::Pack;

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct BuildArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    /// Archive format: zip | tar.gz
    #[arg(long, default_value = "zip", value_name = "FMT")]
    pub format: String,

    /// Output directory (default: build/)
    #[arg(long, value_name = "DIR")]
    pub out: Option<PathBuf>,

    /// Exclude human/ assets (machine-only distribution)
    #[arg(long)]
    pub no_human: bool,

    /// Regenerate machine/mcp_manifest.json before packaging
    #[arg(long)]
    pub gen_mcp_manifest: bool,
}

pub async fn run(args: BuildArgs, _cli: &CmdCtx) -> Result<()> {
    let pack_dir = args.pack.canonicalize()?;
    let pack = Pack::open(&pack_dir)?;
    let manifest = &pack.manifest;

    let slug = manifest.name
        .trim_start_matches('@')
        .replace('/', "-")
        .to_lowercase()
        .replace(' ', "-");
    let version = &manifest.version;
    let base_name = format!("{}-v{}", slug, version);

    let out_dir = args.out.unwrap_or_else(|| pack_dir.join("build"));
    fs::create_dir_all(&out_dir)?;

    let files = collect_files(&pack_dir, args.no_human)?;

    // Compute checksums for all included files
    let mut checksums: BTreeMap<String, String> = BTreeMap::new();
    for rel in &files {
        let hash = sha256_file(&pack_dir.join(rel))?;
        checksums.insert(rel.clone(), hash);
    }
    let checksums_json = serde_json::to_string_pretty(&checksums)?;

    // Write checksums.json alongside the archive
    let checksums_path = out_dir.join("checksums.json");
    fs::write(&checksums_path, &checksums_json)?;

    let archive_name = format!("{}.{}", base_name, args.format);
    let archive_path = out_dir.join(&archive_name);

    eprintln!("Building {}...", archive_name);
    eprintln!("  Including: {}", included_layers(&pack_dir, args.no_human));
    eprintln!("  Excluding: build/");

    match args.format.as_str() {
        "zip" => write_zip(
            &pack_dir,
            &files,
            &checksums_json,
            &base_name,
            &archive_path,
        )?,
        "tar.gz" => write_tar_gz(
            &pack_dir,
            &files,
            &checksums_json,
            &base_name,
            &archive_path,
        )?,
        other => anyhow::bail!("unsupported format '{}' — supported: zip, tar.gz", other),
    }

    println!("  Written:  {}", archive_path.display());
    println!("            {}", checksums_path.display());
    println!("  Files:    {}", files.len() + 1); // +1 for checksums.json inside archive
    Ok(())
}

// ── File collection ──────────────────────────────────────────────────────────

const EXCLUDE_DIRS: &[&str] = &["build", "__pycache__", ".git", "node_modules", "target"];

fn collect_files(pack_dir: &Path, no_human: bool) -> Result<Vec<String>> {
    let mut files: Vec<String> = Vec::new();
    collect_recursive(pack_dir, pack_dir, no_human, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_recursive(root: &Path, dir: &Path, no_human: bool, out: &mut Vec<String>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        if path.is_dir() {
            if EXCLUDE_DIRS.contains(&name.as_str()) {
                continue;
            }
            if no_human && name == "human" {
                continue;
            }
            collect_recursive(root, &path, no_human, out)?;
        } else {
            let rel = path
                .strip_prefix(root)
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_default();
            out.push(rel);
        }
    }
    Ok(())
}

fn included_layers(pack_dir: &Path, no_human: bool) -> String {
    let mut layers = vec!["machine/", "evidence/", "manifest.json", "checksums.json"];
    if !no_human && pack_dir.join("human").exists() {
        layers.push("human/");
    }
    if pack_dir.join("okf").exists() {
        layers.push("okf/");
    }
    layers.join(", ")
}

// ── SHA-256 ──────────────────────────────────────────────────────────────────

fn sha256_file(path: &Path) -> Result<String> {
    let data = fs::read(path)?;
    let hash = Sha256::digest(&data);
    Ok(format!("{:x}", hash))
}

// ── Archive writers ──────────────────────────────────────────────────────────

fn write_zip(
    pack_dir: &Path,
    files: &[String],
    checksums_json: &str,
    base_name: &str,
    out: &Path,
) -> Result<()> {
    let file = fs::File::create(out)?;
    let mut zip = ZipWriter::new(BufWriter::new(file));
    let opts = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for rel in files {
        let archive_path = format!("{}/{}", base_name, rel);
        zip.start_file(&archive_path, opts)?;
        zip.write_all(&fs::read(pack_dir.join(rel))?)?;
    }

    // checksums.json at the archive root
    zip.start_file(format!("{}/checksums.json", base_name), opts)?;
    zip.write_all(checksums_json.as_bytes())?;

    zip.finish()?;
    Ok(())
}

fn write_tar_gz(
    pack_dir: &Path,
    files: &[String],
    checksums_json: &str,
    base_name: &str,
    out: &Path,
) -> Result<()> {
    let file = fs::File::create(out)?;
    let enc = GzEncoder::new(BufWriter::new(file), Compression::best());
    let mut tar = tar::Builder::new(enc);

    for rel in files {
        let abs = pack_dir.join(rel);
        let archive_path = format!("{}/{}", base_name, rel);
        let mut f = fs::File::open(&abs)?;
        tar.append_file(&archive_path, &mut f)?;
    }

    // checksums.json
    let ck_bytes = checksums_json.as_bytes();
    let mut header = tar::Header::new_gnu();
    header.set_size(ck_bytes.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    tar.append_data(
        &mut header,
        format!("{}/checksums.json", base_name),
        ck_bytes,
    )?;

    tar.finish()?;
    Ok(())
}
