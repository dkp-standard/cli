use anyhow::Result;
use clap::{Args, Subcommand};
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

use comfy_table::{presets::UTF8_FULL, Table};
use dkp_core::Pack;

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct GraphArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    #[command(subcommand)]
    pub command: GraphCommands,
}

#[derive(Subcommand, Debug)]
pub enum GraphCommands {
    /// Print node and edge counts by type
    Stats,
    /// Check all edges resolve to known concept IDs (Gate 4)
    Validate,
    /// List all nodes with their type and id
    List {
        #[arg(long, value_name = "TYPE")]
        r#type: Option<String>,
    },
}

pub async fn run(args: GraphArgs, _cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;

    if !pack.has_knowledge_graph() {
        println!("knowledge_graph.json not present in this pack.");
        return Ok(());
    }

    let graph = pack.load_graph()?.expect("presence checked above");

    match args.command {
        GraphCommands::Stats => {
            let mut node_counts: BTreeMap<String, usize> = BTreeMap::new();
            for node in &graph.nodes {
                *node_counts.entry(node.node_type.clone()).or_insert(0) += 1;
            }
            let mut edge_counts: BTreeMap<String, usize> = BTreeMap::new();
            for edge in &graph.edges {
                let label = format!("{:?}", edge.relation).to_lowercase();
                *edge_counts.entry(label).or_insert(0) += 1;
            }

            println!(
                "Knowledge Graph — {} nodes, {} edges\n",
                graph.nodes.len(),
                graph.edges.len()
            );
            println!("Nodes by type:");
            for (t, n) in &node_counts {
                println!("  {t:<20} {n}");
            }
            println!("\nEdges by relation:");
            for (r, n) in &edge_counts {
                println!("  {r:<20} {n}");
            }
        }

        GraphCommands::Validate => {
            let node_ids: HashSet<&str> = graph.nodes.iter().map(|n| n.id.as_str()).collect();
            let mut broken: Vec<String> = Vec::new();
            for edge in &graph.edges {
                if !node_ids.contains(edge.source.as_str()) {
                    broken.push(format!(
                        "unknown source '{}' in edge {}->{}",
                        edge.source, edge.source, edge.target
                    ));
                }
                if !node_ids.contains(edge.target.as_str()) {
                    broken.push(format!(
                        "unknown target '{}' in edge {}->{}",
                        edge.target, edge.source, edge.target
                    ));
                }
            }
            if broken.is_empty() {
                println!(
                    "[PASS] knowledge_graph — {} nodes, {} edges, all edges resolve",
                    graph.nodes.len(),
                    graph.edges.len()
                );
            } else {
                println!(
                    "[FAIL] knowledge_graph — {} broken edge reference(s):",
                    broken.len()
                );
                for b in &broken {
                    println!("  {b}");
                }
                anyhow::bail!("knowledge_graph validation failed");
            }
        }

        GraphCommands::List { r#type: filter } => {
            let nodes: Vec<_> = graph
                .nodes
                .iter()
                .filter(|n| {
                    filter
                        .as_deref()
                        .is_none_or(|t| n.node_type.eq_ignore_ascii_case(t))
                })
                .collect();

            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_header(["ID", "Type", "Label"]);
            for node in &nodes {
                table.add_row([&node.id, &node.node_type, &node.label]);
            }
            println!("{table}");
            println!("\n{} node(s)", nodes.len());
        }
    }

    Ok(())
}
