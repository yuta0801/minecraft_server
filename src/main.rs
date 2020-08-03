use std::io;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handler(mut stream: &TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];

    loop {
        println!("read");
        let n = stream.read(&mut buf)?;
        println!("{:?}", &buf[0..128]);
        if n == 0 {
            println!("end");
            break;
        }
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
