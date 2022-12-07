use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about=Some("listen TCP stream at the spacified port"), long_about=None)]
struct Args {
    #[arg(short, long, default_value_t = 3456)]
    port: u16,
}

pub fn decide_port() -> u16 {
    Args::parse().port
}
