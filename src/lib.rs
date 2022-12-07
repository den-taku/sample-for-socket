use clap::Parser;
use std::net::{SocketAddr, ToSocketAddrs};

#[derive(Parser, Debug)]
#[command(author, version, about=Some("listen TCP stream at the spacified port"), long_about=None)]
struct Args {
    #[arg(short, long, default_value_t = 3456)]
    port: u16,
    #[arg(short, long, default_value_t = false)]
    ipv6: bool,
}

pub fn read_args() -> (u16, bool) {
    (Args::parse().port, Args::parse().ipv6)
}

pub fn generate_address(host: &str, port: u16, use_ipv6: bool) -> std::io::Result<SocketAddr> {
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

    Ok(if use_ipv6 { addr_ipv6 } else { addr_ipv4 })
}
