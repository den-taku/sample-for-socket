use std::net::ToSocketAddrs;

fn main() {
    // host name
    let host = "localhost";
    // port
    let port = 3456;
    // generate SocketAddr (ref: https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html)
    let addrs_iter = (host, port).to_socket_addrs();
    println!("{:?}", addrs_iter);
}
