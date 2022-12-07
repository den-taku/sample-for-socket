# test-socket

sample code for socket communication with `std::net` in Rust

## server (using TCP)

receive message and parrot back

```
cargo run --release --bin server -- -p {port}
```

if you want to use IPv6, use `ipv6` flag

```
cargo run --release --bin server -- --p {port} --ipv6
```

## client (use TCP)

send message to server and read returning message

if you want to halt process, you can send `EOF`

```
cargo run --release --bin client -- -p {port}
```

if you want to use IPv6, use `ipv6` flag

```
cargo run --release --bin client -- --p {port} --ipv6
```

## server (using UDP)

receive message and parrot back

```
cargo run --release --bin server-udp -- -p {port}
```

if you want to use IPv6, use `ipv6` flag

```
cargo run --release --bin server-udp -- --p {port} --ipv6
```

## client (use UDP)

send message to server and read returning message

*Notice that you have to specify the server port*

if you want to halt process, you can send `EOF`

```
cargo run --release --bin client-udp -- -p {port} -s {srver_port}
```

if you want to use IPv6, use `ipv6` flag

```
cargo run --release --bin client-udp -- --p {port} -s {server_port}  --ipv6
```
