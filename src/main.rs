use std::io;
use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::string;

fn read_var_int(reader: &mut BufReader<&TcpStream>) -> io::Result<i32> {
    const MORE_FLAG: u8 = 0b10000000;
    let mut int = 0i32;
    let mut num_read = 0;

    loop {
        let mut buf = [0];
        reader.read_exact(&mut buf)?;
        let byte = buf[0];

        int += ((byte & !MORE_FLAG) as i32) << (7 * num_read);

        num_read += 1;
        if num_read > 5 {
            return Err(io::Error::new(io::ErrorKind::Other, "VarInt is too big"));
        }

        if byte & MORE_FLAG == 0 {
            break;
        }
    }

    Ok(int)
}

// TODO: range check
fn read_string(
    reader: &mut BufReader<&TcpStream>,
) -> io::Result<Result<String, string::FromUtf8Error>> {
    let len = read_var_int(reader)? as usize;
    let mut buf = vec![0; len];
    reader.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf))
}

fn read_unsigned_short(reader: &mut BufReader<&TcpStream>) -> io::Result<u16> {
    let mut buf = [0; 2];
    reader.read_exact(&mut buf)?;
    Ok(((buf[0] as u16) << 8) + buf[1] as u16)
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
            let version = read_var_int(&mut reader)?;
            let _address = read_string(&mut reader)?.unwrap_or("invalid".to_string());
            let _port = read_unsigned_short(&mut reader)?;
            let state = read_var_int(&mut reader)?;
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
