use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;

use crate::packet::{RAuthRes, RExecRes, RconPacket, RconPacketGen};

pub struct RemoteConnection {
    stream: TcpStream,
    packet_gen: RconPacketGen,
    logged_in: bool,
}

impl RemoteConnection {
    /// address is host + port
    pub fn from_address(address: &str) -> Self {
        Self {
            stream: TcpStream::connect(address).unwrap(),
            packet_gen: RconPacketGen::new(),
            logged_in: false,
        }
    }

    fn get_response(&mut self) -> [u8; 4096] {
        // TODO: handle ID uniqueness checks
        let mut buf = [0; 4096];

        // TODO: Handle responses that don't fit into the buffer
        let _ = self.stream.read(&mut buf).unwrap();

        buf
    }

    pub fn authenticate(&mut self, password: String) {
        let auth = self.packet_gen.gen_auth(password);
        // TODO: make sure everything is written
        let _ = self.stream.write(&auth.packet_data()).unwrap();
        let response = self.get_response();
        let log_res = RconPacket::<RAuthRes>::new(response);
        if log_res.is_logged_in() {
            println!("Logged into server succesfully...");
            self.logged_in = true;
        } else {
            println!("Authentication to server failed. Check password.");
            exit(1);
        }
    }

    pub fn send_command(&mut self, command: String) {
        if !self.logged_in {
            panic!("Cannot send a command when not logged in");
        }

        let exec = self.packet_gen.gen_exec(command);
        // TODO: make sure everything is written
        let _ = self.stream.write(&exec.packet_data()).unwrap();
        let response = self.get_response();
        let res = RconPacket::<RExecRes>::new(response);
        println!("{}", res.body());
    }
}
