// Allow clippy lints that are widespread in this tool (to be cleaned up separately)
#![allow(
    unused_imports,
    clippy::cmp_owned,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::type_complexity,
    clippy::manual_pattern_char_comparison,
    clippy::unnecessary_map_or,
    clippy::for_kv_map
)]

//! MEI ODD → Rust Code Generator
//!
//! Parses MEI ODD specification files and generates Rust model types
//! for 1:1 MEI mapping.

mod ast;
mod generator;
mod parser;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mei-codegen")]
#[command(about = "Generate Rust model code from MEI ODD specification")]
struct Args {
    /// Input directory containing MEI ODD module files
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory for generated Rust code
    #[arg(short, long)]
    output: PathBuf,

    /// tusk-mei crate src directory for generating attribute trait impls
    #[arg(long)]
    mei_crate: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("MEI ODD → Rust Code Generator");
    println!("Input:  {}", args.input.display());
    println!("Output: {}", args.output.display());

    // Parse all ODD files
    let defs = parser::parse_odd_files(&args.input)?;

    println!("\nCollected:");
    println!("  Data types:        {}", defs.data_types.len());
    println!("  Pattern entities:  {}", defs.pattern_entities.len());
    println!("  Attribute classes: {}", defs.att_classes.len());
    println!("  Model classes:     {}", defs.model_classes.len());
    println!("  Elements:          {}", defs.elements.len());

    // Generate Rust code
    generator::generate_all(&defs, &args.output)?;

    // Generate attribute and element trait impls for tusk-mei if requested
    if let Some(mei_crate) = &args.mei_crate {
        println!("\nGenerating trait impls for tusk-mei...");
        println!("  Target: {}", mei_crate.display());
        generator::generate_mei_attr_impls(&defs, mei_crate)?;
        generator::generate_mei_element_ser_impls(&defs, mei_crate)?;
        generator::generate_mei_element_deser_impls(&defs, mei_crate)?;
    }

    println!("\nCode generation complete!");

    Ok(())
}
