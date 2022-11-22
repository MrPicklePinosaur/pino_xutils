<div align="center">

# pino_xmodmap

tiny xmodmap parsing library

[![crates.io](https://img.shields.io/crates/v/pino_xmodmap.svg)](https://crates.io/crates/pino_xmodmap)
[![docs.rs](https://docs.rs/pino_xmodmap/badge.svg)](https://docs.rs/pino_xmodmap)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

Requires `xmodmap` to be installed on the system as this library directly calls
it and parses the output. Using this library is very simple:
```rust
use pino_xmodmap::{KeyTable, Modifier, KeySym};

fn main() {
    let xmodmap = KeyTable::new().unwrap();
    let a_key = xmodmap.get_key(KeySym::KEY_a).unwrap();
}
```
