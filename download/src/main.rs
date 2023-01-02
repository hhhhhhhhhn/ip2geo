use std::net::IpAddr;
use csv;
use serde::Deserialize;
use rmp_serde;
use std::fs;
use ip2geo::IPEntry;

#[derive(Deserialize, Debug, Clone)]
pub struct IPEntryCSV {
    pub registry: String,
    pub country: String,
    pub iptype: String,
    pub start: IpAddr,
    pub value: String,
    pub date: String,
    pub status: String,
    pub extensions: String,
}

fn read_file_entries(file_path: &str) -> Vec<IPEntry> {
    let mut r = csv::ReaderBuilder::new()
        .delimiter("|".bytes().nth(0).unwrap())
        .comment("#".bytes().nth(0))
        .flexible(true)
        .has_headers(false)
        .from_path(file_path)
        .unwrap();
    let mut entries = Vec::new();
    for r in r.deserialize::<IPEntryCSV>() {
        match r {
            Err(e) => eprintln!("{}", e),
            Ok(value) => entries.push(IPEntry{start: value.start, country: value.country}),
        }
    }
    return entries
}

fn read_all_entries() -> Vec<IPEntry> {
    let mut entries = [
        "../ipdb/delegated-afrinic-extended-latest",
        "../ipdb/delegated-apnic-extended-latest",
        "../ipdb/delegated-arin-extended-latest",
        "../ipdb/delegated-lacnic-extended-latest",
        "../ipdb/delegated-ripencc-extended-latest",
    ]
        .iter()
        .map(|file_name| read_file_entries(file_name))
        .flatten()
        .collect::<Vec<_>>();

    entries.sort_by(|a, b| a.start.cmp(&b.start));

    return entries
}

fn main() {
    let entries = read_all_entries();
    let mut writer = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("../embed/ipentries.bin")
        .unwrap();
    rmp_serde::encode::write(&mut writer, &entries).unwrap();
}
