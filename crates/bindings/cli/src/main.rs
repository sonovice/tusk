//! Tusk CLI - MusicXML ↔ MEI converter command-line tool.

use clap::Parser;

/// Tusk: The Ultimate Score Konverter
///
/// Bidirectional MusicXML ↔ MEI converter
#[derive(Parser, Debug)]
#[command(name = "tusk")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file (MEI or MusicXML)
    #[arg(value_name = "INPUT")]
    input: Option<std::path::PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    match cli.input {
        Some(path) => {
            println!(
                "Tusk: input file {:?} (conversion not yet implemented)",
                path
            );
        }
        None => {
            println!("Tusk: The Ultimate Score Konverter");
            println!("Use --help for usage information");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_parses_no_args() {
        let cli = Cli::parse_from::<[&str; 1], _>(["tusk"]);
        assert!(cli.input.is_none());
    }
}
