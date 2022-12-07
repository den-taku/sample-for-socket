# test-socket

sample code for socket communication with `std::net` in Rust

## server

receive message and parrot back

```
cargo run --release --bin server -- -p {port}
```

if you want to use IPv6, use `ipv6` flag

```
cargo run --release --bin server -- --p {port} --ipv6
```

## client

send message to server and read returning message

if you want to halt process, you can send `EOF`

```
cargo run --release --bin server -- -p {port}
```

if you want to use IPv6, use `ipv6` flag

```
cargo run --release --bin client -- --p {port} --ipv6
```
