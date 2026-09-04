#[allow(unused_imports)]
use std::net::TcpListener;
use std::thread;
use std::{io::Write, net::TcpStream};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let response = b"HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response)?;
    stream.flush()
}

fn main() -> std::io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprint!("Error handling clieent: {e}");
                    }
                });
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }

    Ok(())
}
