use super::COMMAND_LIST;
use log::{debug, info};
use std::process::Command;

pub(crate) fn say_hello() {
    info!("Hello, World!");
}

pub(crate) fn echo(args: Vec<String>) {
    debug!("{}", args.join(" "));
    info!("{}", args.join(" "))
}

pub(crate) fn exit() {
    debug!("Exiting...");
    std::process::exit(0)
}

pub(crate) fn clear() {
    info!("Clearing screen..., running command");
    let _result = if cfg!(target_os = "windows") {
        debug!("target_os is windows");
        Command::new("cmd").args(["/c", "cls"]).spawn()
    } else {
        debug!("target_os is unix");
        // "clear" or "tput reset"
        Command::new("tput").arg("reset").spawn()
    };
}

pub(crate) fn help() {
    println!("Commands:");
    for cmd in COMMAND_LIST.commands.read().iter() {
        println!("{:#}", cmd);
    }
}
