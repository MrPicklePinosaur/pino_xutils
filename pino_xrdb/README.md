<div align="center">

# pino_xrdb

simple zero-dependency rust parser for `xrdb`

[![crates.io](https://img.shields.io/crates/v/pino_xrdb.svg)](https://crates.io/crates/pino_xrdb)
[![docs.rs](https://docs.rs/pino_xrdb/badge.svg)](https://docs.rs/pino_xrdb)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

Requires `xrdb` to be installed on the system as this library directly calls
`xrdb` and parses it.

It is very simple to query the `xrdb` of the system.
```rust
use pino_xrdb::Xrdb;
fn main() {

    let mut xrdb = Xrdb::new();
    xrdb.read().unwrap();
    
    if let Some(value) = xrdb.query("dwm", "color1") {
        println!("dwm.color1 has value {}", value);
    } else {
        println!("dwm.color1 not found");
    }
    
}
```
