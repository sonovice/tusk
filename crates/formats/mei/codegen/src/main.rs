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

//! MEI ODD → Rust Code Generator (tusk-mei format codegen).
//!
//! Parses MEI ODD specification files and generates Rust model types
//! for 1:1 MEI mapping. Lives under crates/formats/mei/codegen; no cross-format deps.

mod ast;
mod generator;
mod parser;
mod rng;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tusk-mei-codegen")]
#[command(about = "Generate Rust model and MEI ser/de code from MEI ODD or RNG specification")]
struct Args {
    /// Input directory containing MEI ODD module files (required when --rng is not set)
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Output directory for generated Rust code (tusk-model generated/)
    #[arg(short, long)]
    output: PathBuf,

    /// tusk-mei crate src directory for generating attribute/element trait impls (skip when --versioned)
    #[arg(long)]
    mei_crate: Option<PathBuf>,

    /// Use MEI RNG file (e.g. codegen/schema/versions/mei-all_v6.0-dev.rng). When set, --input is not required.
    #[arg(long)]
    rng: Option<PathBuf>,

    /// Generate a versioned import model; use with --rng pointing to codegen/schema/versions/mei-all_vX.Y.rng. Output should be crates/formats/mei/src/versions/<label>. Module path will be crate::versions::<label>.
    #[arg(long)]
    versioned: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let defs = if let Some(rng_path) = &args.rng {
        println!("MEI RNG → Rust Code Generator");
        println!("RNG:    {}", rng_path.display());
        println!("Output: {}", args.output.display());
        rng::parse_rng_file(rng_path)?
    } else {
        let input = args.input.ok_or_else(|| {
            anyhow::anyhow!("Either --input (ODD dir) or --rng (RNG file) is required")
        })?;
        println!("MEI ODD → Rust Code Generator");
        println!("Input:  {}", input.display());
        println!("Output: {}", args.output.display());
        parser::parse_odd_files(&input)?
    };

    println!("\nCollected:");
    println!("  Data types:        {}", defs.data_types.len());
    println!("  Pattern entities:  {}", defs.pattern_entities.len());
    println!("  Attribute classes: {}", defs.att_classes.len());
    println!("  Model classes:     {}", defs.model_classes.len());
    println!("  Elements:          {}", defs.elements.len());

    let config = if let Some(ref label) = args.versioned {
        if args.rng.is_none() {
            anyhow::bail!(
                "--versioned requires --rng (versioned models are generated from RNG only)"
            );
        }
        let module_path = format!("crate::versions::{}", label);
        println!("  Versioned module:  {}", module_path);
        generator::CodegenConfig { module_path }
    } else {
        generator::CodegenConfig::default()
    };

    // Generate Rust code
    generator::generate_all_with_config(&defs, &args.output, &config)?;

    // Generate attribute and element trait impls for tusk-mei if requested (only for main model, not versioned)
    if args.versioned.is_none() {
        if let Some(mei_crate) = &args.mei_crate {
            println!("\nGenerating trait impls for tusk-mei...");
            println!("  Target: {}", mei_crate.display());
            generator::generate_mei_attr_impls(&defs, mei_crate)?;
            generator::generate_mei_element_ser_impls(&defs, mei_crate)?;
            generator::generate_mei_element_deser_impls(&defs, mei_crate)?;
        }
    }

    println!("\nCode generation complete!");

    Ok(())
}
