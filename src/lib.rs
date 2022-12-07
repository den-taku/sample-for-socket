use clap::Parser;

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
