use std::net::ToSocketAddrs;

fn main() -> std::io::Result<()> {
    // host name (maybe 127.0.0.1)
    let host = "localhost";
    // port
    let port = 3456;

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

    Ok(())
}
