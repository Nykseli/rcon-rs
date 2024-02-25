use std::env;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Commandline tool for sending RCON commands
pub struct Args {
    /// Name of the server in config
    pub server: String,

    /// Server hostname or ip address
    #[arg(short, long)]
    pub address: Option<String>,

    /// Target TCP port
    #[arg(short, long)]
    pub port: Option<u16>,

    /// Password for authenticating the RCON connection
    #[arg(short, long)]
    pub secret: Option<String>,

    /// File containing commands. If file is not specified, interactive mode is used
    #[arg(short, long)]
    pub file: Option<String>,

    /// Path to a config file. Defaults to '~/.rcon.toml
    #[arg(short, long)]
    config: Option<String>,
}

impl Args {
    pub fn config(&self) -> String {
        match &self.config {
            Some(conf) => conf.clone(),
            None => {
                let home = env::var("HOME").expect("HOME not found in env");
                format!("{home}/.rcon.toml")
            }
        }
    }
}
