static DICTIONARY_DATA: &[u8] = include_bytes!("../assets/mamoru.fst");

mod cli;

use clap::Parser;
use cli::{Cli, Commands, Format, check_commit, initialization, uninstall};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Init { force } => {
            initialization(force)?;
        }
        Commands::Uninstall => {
            uninstall()?;
        }
        Commands::Check {
            path,
            ascii_only,
            format,
        } => {
            let dict = fuzzies::Dictionary::from_embedded(DICTIONARY_DATA)?;
            if let Err(e) = check_commit(&path, &dict, format, ascii_only) {
                match format {
                    Format::Silent => {}
                    Format::Brief | Format::Long => {
                        eprintln!("{}", e)
                    }
                }
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
