use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // host name (maybe 127.0.0.1)
    let host = "localhost";
    // port
    let (port, use_ipv6) = test_socket::read_args();

    let addr = test_socket::generate_address(host, port, use_ipv6)?;

    // client
    let mut stream = TcpStream::connect(addr)?;

    // send message
    let mut message = String::new();
    let mut buffer = [0; 256];
    loop {
        message.clear();
        // read message from stdio
        print!("message: ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut message)?;
        if &message == "EOF\n" {
            stream.write("EOF".as_bytes())?;
            break;
        }

        // send to server
        stream.write_all(&message.as_bytes())?;

        // receive message from server
        let size = stream.read(&mut buffer)?;
        let message = match String::from_utf8(buffer[0..size].to_vec()) {
            Ok(message) => message,
            Err(e) => {
                println!("parsing error: {e:}");
                "".to_string()
            }
        };
        print!("returning message: {message}");
    }

    Ok(())
}
