mod cli;
mod config;
mod traits;
mod error;
mod compiler;
mod result_store;
mod results;
mod state_manager;
mod hasher;
mod rustc_options;
mod crate_discovery;
mod layer_manager;
mod compilation_orchestrator;
mod app_context;
mod app_builder;
mod app_runner;

use cli::Args;
use app_builder::AppBuilder;
use app_runner::AppRunner;
use error::AppError;

fn main() -> Result<(), AppError> {
    let args = Args::parse_args();
    let context = AppBuilder::build_from_args(args.clone())?;
    AppRunner::run(context, args)
}
