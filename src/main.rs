mod cli;

use clap::Parser;
use cli::{Args, initialization};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.init {
        if let Err(e) = initialization(&args) {
            eprintln!("Initialization failed: {}", e);
            std::process::exit(1);
        }
        return Ok(());
    }

    Ok(())
}
