use std::io::{self, BufRead, Write};
use std::process::exit;

use connection::RemoteConnection;

use crate::command_file::commands_from_file;

mod command_file;
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
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: cli <host:ip> <password> [commandfile]");
        exit(1);
    }

    if args.len() == 3 {
        let mut con = RemoteConnection::from_address(&args[1]);
        con.authenticate(args[2].clone());
        interactive(&mut con, &args[1]);
    } else {
        let commands = commands_from_file(&args[3]);
        let mut con = RemoteConnection::from_address(&args[1]);
        con.authenticate(args[2].clone());
        for command in commands {
            con.send_command(command);
        }
    }
}
