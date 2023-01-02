# ip2geo
A small Rust library for getting the country code of IP addresses.

## Usage
```rust
use ip2geo;
use std::net::IpAddr;

fn main() {
    let address: IpAddr = "152.179.124.137".parse().unwrap();
    let country_code = ip2geo::search(address).unwrap().country;
    println!("{}", country_code);
}
```

## Note on performance
The library takes a few seconds to
parse the embedded binary data.
After that, each search takes almost no time.

## Compilation instructions
First, download the databases:

```bash
cd ipdb
sh download.sh
```

Then, turn them into a compressed binary for embedding:

```
cd download
cargo run
```
