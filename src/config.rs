use std::{collections::HashMap, fs};

use serde::Deserialize;

use crate::cli_args::Args;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    /// Server hostname or ip address
    pub address: String,
    /// Target TCP port
    pub port: u16,
    /// Password for authenticating the RCON connection
    pub secret: String,
}

impl ServerConfig {
    fn new(address: String, port: u16, secret: String) -> Self {
        Self {
            address,
            port,
            secret,
        }
    }

    /// Get the host in <addr>:<port> format
    pub fn full_host(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }

    pub fn from_args(args: &Args) -> Self {
        // If all config options are set by cli args, no need to parse configs

        if let (Some(address), Some(port), Some(secret)) = (&args.address, args.port, &args.secret)
        {
            return Self::new(address.into(), port, secret.into());
        }

        let file = args.config();
        let decoded: HashMap<String, ServerConfig> =
            toml::from_str(&fs::read_to_string(file).unwrap()).unwrap();

        let config = decoded
            .get(&args.server)
            .unwrap_or_else(|| panic!("Server named '{}' was not found from config.", args.server));

        let port = args.port.unwrap_or(config.port);
        let secret = args.secret.as_ref().unwrap_or(&config.secret).into();
        let address = args.address.as_ref().unwrap_or(&config.address).into();

        Self::new(address, port, secret)
    }
}
