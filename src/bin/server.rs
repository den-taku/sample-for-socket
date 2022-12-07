use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    // host name (maybe 127.0.0.1)
    let host = "localhost";
    // port
    let (port, use_ipv6) = test_socket::read_args();

    let addr = test_socket::generate_address(host, port, use_ipv6)?;

    // server
    let listener = TcpListener::bind(addr)?;
    println!("{:?}", listener);

    for stream in listener.incoming() {
        std::thread::spawn(|| {
            // buffer
            let mut buffer = [0; 256];
            println!("\nINCOMING: {:?}", stream);
            handle_incoming_stream(stream.unwrap(), &mut buffer).unwrap();
            println!("CLOSE\n");
        });
    }
    Ok(())
}

fn handle_incoming_stream(mut stream: TcpStream, buffer: &mut [u8]) -> std::io::Result<()> {
    // stream.set_read_timeout(None)?;

    // receive messages until receive EOF
    loop {
        // read from stream
        let size = stream.read(buffer)?;
        // parse
        let message = match String::from_utf8(buffer[0..size].to_vec()) {
            Ok(message) => message,
            Err(e) => {
                println!("parsing error: {e:}");
                "".to_string()
            }
        };
        println!("incoming message: {message:?} from {stream:?}");
        // valid end: sent EOF from client
        // invalid end: read fail (e.g. connection disconnect)
        if &message == "EOF" || &message == "" {
            break;
        }
        // return the same message to client
        stream.write_all(&buffer[0..size])?;
    }

    Ok(())
}
