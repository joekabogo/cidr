Simple Rust utility that calculates start address,end address and broadcast address given an IP CIDR
### Build
```
cargo build --release
```

`./target/release/cidr <IP/cidr>`
### Example
```
./target/release/cidr 10.16.0.0/20
```

```
Start: 10.16.0.0
End: 10.16.15.255
Broadcast: 10.16.15.255
```
