#[macro_use]
extern crate log;
extern crate simplelog;

mod config_handler;
mod input_handler;
mod stratagem_handler;

use std::fs::File;

use chrono::{DateTime, Local};
use clap::Parser;
use log::LevelFilter;
use simplelog::{CombinedLogger, WriteLogger};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    generate: bool,

    #[clap(long, short)]
    config: Option<String>,

    #[clap(long)]
    stratagem: Option<String>,
}

fn main() {
    // Create the logs directory if it does not exist
    std::fs::create_dir_all("logs").unwrap();

    let now: DateTime<Local> = Local::now();

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        simplelog::Config::default(),
        File::create(format!("logs/{}.log", now.format("%Y-%m-%d-%H-%M-%S"))).unwrap(),
    )])
    .unwrap();

    info!("Starting Stratagem Engine");

    let args: Args = Args::parse();

    debug!("Args: {:?}", args);

    let config_handler =
        config_handler::ConfigHandler::new(args.generate).expect("Failed to load config.");

    debug!("Config: {:?}", config_handler.config);

    let stratagem_handler = stratagem_handler::StratagemHandler::new("stratagems.toml".into())
        .expect("Failed to load stratagems.");

    debug!("Stratagems: {:?}", stratagem_handler.stratagems);

    if let Some(stratagem) = args.stratagem {
        stratagem_handler.execute(&stratagem, &config_handler.config);
    }

    info!("Stratagem Engine finished!")
}
