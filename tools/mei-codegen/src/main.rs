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

    println!("\nCode generation complete!");

    Ok(())
}
