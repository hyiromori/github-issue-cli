mod github;
mod subcommand;
mod util;
mod zenhub;

extern crate dirs;
use crate::subcommand::board::board;
use crate::subcommand::config::config;
use crate::util::config::read_config;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let command = String::from(&args[0]);
    let subcommand = String::from(&args[1]);
    let args: Vec<String> = args.split_off(2);

    if subcommand == String::from("config") {
        config(&args).await?;
        return Ok(());
    }

    let config = read_config();
    if config.is_none() {
        panic!(
            "Config file not found or invalid. Configure by `{command} config` command.",
            command = command
        );
    }
    let config = config.unwrap();

    if subcommand == String::from("board") {
        board(&config, &args).await?;
        return Ok(());
    }

    panic!("Undefined subcommand: {:?}", subcommand);
}
