static DICTIONARY_DATA: &[u8] = include_bytes!("../assets/dict.fst");

mod cli;

use clap::Parser;
use cli::{Args, Format, check_commit, initialization, uninstall};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.uninstall {
        if let Err(e) = uninstall() {
            eprintln!("Uninstallation failed: {}", e);
            std::process::exit(1);
        }
        return Ok(());
    }

    if args.init {
        if let Err(e) = initialization(&args) {
            eprintln!("Initialization failed: {}", e);
            std::process::exit(1);
        }
        return Ok(());
    }

    let dict = fuzzies::Dictionary::from_embedded(DICTIONARY_DATA)?;

    if let Err(e) = check_commit(&args.path, &dict, args.format, args.ascii_only) {
        match args.format {
            Format::Silent => {}
            Format::Brief | Format::Long => {
                eprintln!("{}", e);
            }
        }
        std::process::exit(1);
    }

    Ok(())
}
