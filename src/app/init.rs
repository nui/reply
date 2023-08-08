use crate::app::cmdline::Args;
use crate::app::config::Config;

/// Program initialization.
///
/// This function setup thing that doesn't need async runtime.
pub fn initialize(args: Args) -> Initialized {
    let config = Config::load(&args).expect("configuration");
    if args.print_config {
        display_current_configuration(&config);
    }
    crate::app::tracing::init();
    Initialized { args, config }
}

fn display_current_configuration(config: &Config) {
    let config = config.clone();
    // Using eprintln! is intended. We don't want secrets to be in log file!!!
    eprintln!(">> {:-^77}", " Begin Active Configuration ");
    eprintln!(
        "{}",
        config.to_toml_pretty().expect("configuration in toml")
    );
    eprintln!(">> {:-^77}", " End Active Configuration ");
}

pub struct Initialized {
    pub args: Args,
    pub config: Config,
}
