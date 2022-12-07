use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use test_socket::decide_port;

fn main() -> std::io::Result<()> {
    // host name (maybe 127.0.0.1)
    let host = "localhost";
    // port
    let port = decide_port();

    // generate SocketAddr (ref: https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html)
    let mut addrs_iter = (host, port).to_socket_addrs()?;
    let addr_ipv6 = addrs_iter
        .find(|addr| addr.is_ipv6())
        .expect("failed to get ipv6 address");
    println!("ipv6 address is {addr_ipv6:?}.");
    let addr_ipv4 = addrs_iter
        .find(|addr| addr.is_ipv4())
        .expect("failed to get ipv4 address");
    println!("ipv4 address is {addr_ipv4:?}.");

    // server
    let listener = TcpListener::bind(addr_ipv4)?;
    println!("{:?}", listener);

    // buffer
    let mut buffer = [0; 256];

    for stream in listener.incoming() {
        println!("\nINCOMING: {:?}", stream);
        handle_incoming_stream(stream?, &mut buffer)?;
        println!("CLOSE\n");
    }
    Ok(())
}

fn handle_incoming_stream(mut stream: TcpStream, buffer: &mut [u8]) -> std::io::Result<()> {
    stream.set_read_timeout(None)?;
    // clear buffer
    for b in buffer.into_iter() {
        *b = 0;
    }

    // read from stream
    let size = stream.read(buffer)?;
    let message = match String::from_utf8(buffer[0..size].to_vec()) {
        Ok(message) => message,
        Err(e) => {
            println!("parsing error: {e:}");
            "".to_string()
        }
    };
    println!("incoming message: {message:?}");

    // return the same message to client
    stream.write_all(&buffer[0..size])
}
