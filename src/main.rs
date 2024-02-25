use clap::Parser;
use config::ServerConfig;
use std::io::{self, BufRead, Write};

use cli_args::Args;
use connection::RemoteConnection;

use crate::command_file::commands_from_file;

mod cli_args;
mod command_file;
mod config;
mod connection;
mod packet;

fn interactive(connection: &mut RemoteConnection, host: &str) {
    let stdin = io::stdin();

    loop {
        print!("{host}> ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        {
            let len = stdin.lock().read_line(&mut command).unwrap();
            if len == 0 {
                break;
            }

            connection.send_command(command);
        }
    }

    println!("Goodbye!");
}

fn main() {
    let args = Args::parse();
    let config = ServerConfig::from_args(&args);
    let host = config.full_host();

    match args.file {
        Some(file) => {
            let commands = commands_from_file(&file);
            let mut con = RemoteConnection::from_address(&host);
            con.authenticate(config.secret);
            for command in commands {
                con.send_command(command);
            }
        }
        None => {
            let mut con = RemoteConnection::from_address(&host);
            con.authenticate(config.secret);
            interactive(&mut con, &host);
        }
    }
}
