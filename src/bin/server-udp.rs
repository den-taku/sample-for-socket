use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // host name (maybe 127.0.0.1)
    let host = "localhost";
    // port
    let (port, use_ipv6) = test_socket::read_args();

    let addr = test_socket::generate_address(host, port, use_ipv6)?;

    // server
    let listener = UdpSocket::bind(addr)?;
    println!("{:?}", listener);

    // buffer
    let mut buffer = [0; 256];

    loop {
        let (size, src) = listener.recv_from(&mut buffer)?;
        println!("\nINCOMING from {:?}", src);

        let message = match String::from_utf8(buffer[0..size].to_vec()) {
            Ok(message) => message,
            Err(e) => {
                println!("parsing error: {e:}");
                "".to_string()
            }
        };
        println!("incoming message: {message:?}");

        // valid end: sent EOF from client
        // invalid end: read fail (e.g. connection disconnect)
        if &message == "EOF" || &message == "" {
            break;
        }
        // return the same message to client
        listener.send_to(&buffer[..size], &src)?;
    }

    Ok(())
}
