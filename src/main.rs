use std::process::exit;

use connection::RemoteConnection;

mod connection;
mod packet;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: cli <host:ip> <password>");
        exit(1);
    }

    let mut con = RemoteConnection::from_address(&args[1]);
    con.authenticate(args[2].clone());
    con.send_command("status".into());
}
