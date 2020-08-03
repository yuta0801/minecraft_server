mod reader;

use std::io;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

fn handler(stream: &TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream);

    let _len = reader::read_var_int(&mut reader)?;
    let packet_id = reader::read_var_int(&mut reader)?;
    println!("packet length: {:?}", _len);
    println!("packet id: {:?}", packet_id);

    match packet_id {
        // Handshake
        0x00 => {
            let version = reader::read_var_int(&mut reader)?;
            let _address = reader::read_string(&mut reader)?.unwrap_or("invalid".to_string());
            let _port = reader::read_unsigned_short(&mut reader)?;
            let state = reader::read_var_int(&mut reader)?;
            println!("packet: handshake");
            println!("protocol version: {}", version);
            println!("server address: {}", _address);
            println!("server port: {}", _port);
            println!("next state: {}", state);
        }
        // othors
        _ => println!("unknown packet: {}", packet_id),
    }

    Ok(())
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:25565").expect("failed to bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handler(&stream).unwrap_or_else(|e| println!("{:?}", e)),
            Err(e) => {
                println!("error: {}", e);
                continue;
            }
        };
    }
}
