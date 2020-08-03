use std::io;
use std::io::{Read, BufReader};
use std::net::{TcpListener, TcpStream};

// TODO: support multibyte int
fn read_var_int(reader: &mut BufReader<&TcpStream>) -> io::Result<i32> {
    let mut buf = [0];
    reader.read_exact(&mut buf)?;
    Ok(buf[0] as i32)
}

fn handler(stream: &TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream);

    let _len = read_var_int(&mut reader)?;
    let packet_id = read_var_int(&mut reader)?;
    println!("packet length: {:?}", _len);
    println!("packet id: {:?}", packet_id);
    
    match packet_id {
        // Handshake
        0x00 => {
            // let version = read_var_int(&mut reader);
            println!("packet: handshake");
        },
        // othors
        _ => {},
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
