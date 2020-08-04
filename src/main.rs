mod reader;
mod player;

use std::io;
use std::io::{Read, BufReader};
use std::net::{TcpListener, TcpStream};
use player::Player;

fn packet_handler<R: Read>(mut reader: R, player: &mut Player) -> io::Result<()> {
    let packet_id = reader::read_var_int(&mut reader)?;

    match packet_id {
        // Handshake
        0x00 => {
            match player.state {
                0 => {
                    let version = reader::read_var_int(&mut reader)?;
                    let _address = reader::read_string(&mut reader)?.unwrap_or("invalid".to_string());
                    let _port = reader::read_unsigned_short(&mut reader)?;
                    let state = reader::read_var_int(&mut reader)?;
                    println!("packet: handshake");
                    println!("protocol version: {}", version);
                    println!("server address: {}", _address);
                    println!("server port: {}", _port);
                    println!("next state: {}", state);
                    player.state = state;
                }
                1 => {
                    println!("packet: status request")
                }
                n => println!("invalid state: {}", n)
            }
        }
        // othors
        _ => println!("unknown packet: {}", packet_id),
    }

    Ok(())
}

fn handler(stream: &TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut player = Player { state: 0 };

    loop {
        let len = reader::read_var_int(&mut reader)?;
        println!("packet length: {:?}", len);
        let mut packet = vec![0; len as usize];
        reader.read_exact(&mut packet)?;
        packet_handler(packet.as_slice(), &mut player)?;
    }
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
