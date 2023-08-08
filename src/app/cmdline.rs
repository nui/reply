use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap()]
pub struct Args {
    #[clap(
        short,
        long,
        env = "REPLY_CONFIG",
        help = "Configuration file (e.g., reply.toml)"
    )]
    pub config: Option<PathBuf>,
    #[clap(long, help = "Prints active configuration")]
    pub print_config: bool,
}

pub fn parse_and_validate() -> Args {
    Args::parse()
}
