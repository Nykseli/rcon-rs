use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Commandline tool for sending RCON commands
pub struct Args {
    /// Server hostname or ip address
    #[arg(short, long)]
    pub address: String,

    /// Target TCP port
    #[arg(short, long)]
    pub port: u16,

    /// Password for authenticating the RCON connection
    #[arg(short, long)]
    pub secret: String,

    /// File containing commands. If file is not specified, interactive mode is used.
    #[arg(short, long)]
    pub file: Option<String>,
}

impl Args {
    pub fn full_host(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}
