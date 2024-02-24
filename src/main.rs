use std::io::{self, BufRead, Write};
use std::process::exit;

use connection::RemoteConnection;

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
        println!("Usage: cli <host:ip> <password>");
        exit(1);
    }

    let mut con = RemoteConnection::from_address(&args[1]);
    con.authenticate(args[2].clone());
    interactive(&mut con, &args[1]);
}
