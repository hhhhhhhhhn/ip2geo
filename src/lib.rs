#[macro_use]
extern crate lazy_static;

use serde::{Serialize, Deserialize};
use std::net::IpAddr;
use rust_embed::RustEmbed;
use rmp_serde;

#[derive(RustEmbed)]
#[folder = "embed/"]
struct Asset;

#[derive(Serialize, Deserialize, Clone)]
pub struct IPEntry {
    pub start: IpAddr,
    pub country: String,
}

lazy_static! {
    pub static ref ENTRIES: Vec<IPEntry> = {
        let data = Asset::get("ipentries.bin").unwrap().data;
        let deserialized: Vec<IPEntry> = rmp_serde::from_slice(data.into_owned().as_slice()).unwrap();
        deserialized
    };
}

pub fn search(address: &IpAddr) -> Option<IPEntry> {
    let search = ENTRIES.binary_search_by_key(address, |entry| entry.start);
    match search {
        Ok(index) => Some(ENTRIES.get(index-1)?.clone()),
        Err(index) => Some(ENTRIES.get(index-1)?.clone()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search() {
        let entry = search(&"152.179.124.137".parse::<IpAddr>().unwrap()).unwrap();
        assert_eq!(entry.country, "US".to_string());

        let entry = search(&"85.214.132.117".parse::<IpAddr>().unwrap()).unwrap();
        assert_eq!(entry.country, "DE".to_string());
    }

    #[test]
    fn test_search_1_000_000() {
        for _ in 0..1_000_000 {
            let entry = search(&"152.179.124.137".parse::<IpAddr>().unwrap()).unwrap();
            assert_eq!(entry.country, "US".to_string());
        }
    }
}
