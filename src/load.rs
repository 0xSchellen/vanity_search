use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::SystemTime;

pub fn load() -> HashMap<String, u8> {
    // 1 - Load addresses file
    println!("1 - Loading addresses file");

    let file = File::open("eth_addresses.csv").expect("Problem opening the file eth_addresses.csv");
    let reader = BufReader::new(file);

    let mut addr_hashmap = HashMap::new();

    let now = SystemTime::now();
    let mut line_count: i32 = 0;

    for line in reader.lines() {
        line_count = line_count + 1;

        let line1 = line.unwrap();
        let split1: Vec<&str> = line1.split('.').collect();

        let line2 = split1[0];
        let split2: Vec<&str> = line2.split(',').collect();

        let address: &str = split2[0];

        // Takes out "0x" string
        let address = address.strip_prefix("0x").unwrap();

        // Get only the first 20 bytes of the address to speed HashMap search
        let address = &address[..20];

        // let balance: u64 = split2[1].parse::<u64>().unwrap();
        let balance: u8 = 1;

        let address_to_insert = String::from(address);

        // println!("{line_count} - {address_to_insert}");

        addr_hashmap.insert(address_to_insert, balance);
    }
    let total_time = now.elapsed().unwrap();
    println!("Total addresses: {} - Time to load data: {} ", line_count, total_time.as_millis() / 1000);

    addr_hashmap
}
