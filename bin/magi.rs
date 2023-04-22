use std::{path::PathBuf, sync::mpsc::channel};

use clap::Parser;
use dirs::home_dir;
use eyre::Result;

use magi::{
    config::{ChainConfig, CliConfig, Config, SyncMode},
    driver::Driver,
    telemetry::{self, metrics},
};
use serde::Serialize;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbose = cli.verbose;
    let logs_dir = cli.logs_dir.clone();
    let logs_rotation = cli.logs_rotation.clone();
    let config = cli.to_config();

    let _guards = telemetry::init(verbose, logs_dir, logs_rotation);
    metrics::init()?;

    if let SyncMode::Challenge = config.sync_mode {
        panic!("challenge sync not implemented yet");
    }

    run_sync(config).await?;

    Ok(())
}

pub async fn run_sync(config: Config) -> Result<()> {
    tracing::info!(target: "magi", "starting full sync");
    let (shutdown_sender, shutdown_recv) = channel();

    let mut driver = Driver::from_config(config, shutdown_recv)?;

    ctrlc::set_handler(move || {
        tracing::info!(target: "magi", "shutting down");
        shutdown_sender.send(true).expect("shutdown failure");
    })
    .expect("could not register shutdown handler");

    // Run the driver
    if let Err(err) = driver.start().await {
        tracing::error!(target: "magi", "{}", err);
        std::process::exit(1);
    }

    Ok(())
}

#[derive(Parser, Serialize)]
pub struct Cli {
    #[clap(short, long, default_value = "optimism-goerli")]
    network: String,
    #[clap(long)]
    data_dir: Option<String>,
    #[clap(long)]
    l1_rpc_url: Option<String>,
    #[clap(long)]
    l2_rpc_url: Option<String>,
    #[clap(short = 'm', long, default_value = "full")]
    sync_mode: SyncMode,
    #[clap(long)]
    l2_engine_url: Option<String>,
    #[clap(long)]
    jwt_secret: Option<String>,
    #[clap(short = 'v', long)]
    verbose: bool,
    #[clap(long)]
    logs_dir: Option<String>,
    #[clap(long)]
    logs_rotation: Option<String>,
}

impl Cli {
    pub fn to_config(self) -> Config {
        let chain = match self.network.as_str() {
            "optimism-goerli" => ChainConfig::optimism_goerli(),
            "base-goerli" => ChainConfig::base_goerli(),
            _ => panic!("network not recognized"),
        };

        let config_path = home_dir().unwrap().join(".magi/magi.toml");
        let cli_config = CliConfig::from(self);
        Config::new(&config_path, cli_config, chain)
    }
}

impl From<Cli> for CliConfig {
    fn from(value: Cli) -> Self {
        Self {
            l1_rpc_url: value.l1_rpc_url,
            l2_rpc_url: value.l2_rpc_url,
            l2_engine_url: value.l2_engine_url,
            jwt_secret: value.jwt_secret,
            data_dir: value.data_dir.map(PathBuf::from),
        }
    }
}
