#![deny(rust_2018_idioms)]

mod app;
mod handlers;
mod routes;

// Top level type alias for Result type
pub type Result<T, E = anyhow::Error> = core::result::Result<T, E>;

fn main() -> Result<()> {
    let args = app::cmdline::parse_and_validate();
    let app::init::Initialized { args: _, config } = app::init::initialize(args);
    let runtime = app::runtime::build()?;
    app::server::run_server(&runtime, config);
    runtime.shutdown_background();
    Ok(())
}
