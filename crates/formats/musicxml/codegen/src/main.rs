//! CLI for MusicXML codegen.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use tusk_musicxml_codegen::{generate, parse_xsd};

#[derive(Parser)]
#[command(name = "tusk-musicxml-codegen")]
#[command(about = "Generate Rust MusicXML model from XSD schema (4.1)")]
struct Args {
    /// Path to musicxml.xsd (e.g. codegen/schema/versions/musicxml-4.1/schema/musicxml.xsd)
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory for generated Rust code
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("MusicXML XSD → Rust Code Generator");
    println!("  Input:  {}", args.input.display());
    println!("  Output: {}", args.output.display());

    let schema = parse_xsd(&args.input)?;
    println!(
        "\nCollected: {} simple types, {} complex types, {} groups, {} attribute groups, {} elements",
        schema.simple_types.len(),
        schema.complex_types.len(),
        schema.groups.len(),
        schema.attribute_groups.len(),
        schema.elements.len(),
    );

    generate(&schema, &args.output)?;
    println!("\nCode generation complete.");
    Ok(())
}
