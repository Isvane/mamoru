use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, Default)]
pub(crate) enum Format {
    Silent,
    Brief,
    #[default]
    Long,
}

#[derive(Debug, Parser)]
#[command(rename_all = "kebab_case")]
#[command(about, author, version)]
pub(crate) struct Args {
    #[arg(long, help = "Install mamoru automatically into .git/hooks/")]
    pub(crate) init: bool,

    #[arg(
        short,
        long,
        requires = "init",
        help = "Overwrite existing hook during init"
    )]
    pub(crate) force: bool,

    #[arg(long, help = "Only use ASCII characters for output")]
    pub(crate) ascii_only: bool,

    #[arg(index = 1, default_value = ".git/COMMIT_EDITMSG")]
    pub(crate) path: PathBuf,

    #[arg(short, long, ignore_case = true, value_enum, default_value_t = Format::Long)]
    pub(crate) format: Format,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
