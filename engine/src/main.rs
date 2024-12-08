#![deny(clippy::unwrap_in_result)]
#![feature(iter_collect_into)]
use anyhow::Result;
use log::LevelFilter;
use plugin_api::plugin_imports::*;
use plugin_api::{get_plugin, PluginManager, load_plugins, Plugin, iterate_plugins};
use std::collections::HashMap;

pub mod core;
pub mod utils;

use utils::{logger::LOGGER, splash::print_splash};

#[tokio::main]
async fn main() -> Result<()> {
    // Load all plugins
    let mut plugin_manager = plugin_api::PluginManager::new();
    let plugins = plugin_manager.load_all();

    // iterate_plugins!(plugins, test);

    log::set_logger(&*LOGGER).ok();
    log::set_max_level(LevelFilter::Off);

    print_splash();
    let mut plugin_manager = PluginManager::new();


    LOGGER.write_to_stdout();

    let shell_thread = tokio::task::spawn(async { core::repl::exec::handle_repl().await });

    core::init_renderer()?;
    shell_thread.await??;

    Ok(())
}
