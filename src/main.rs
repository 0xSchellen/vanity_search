mod load;
mod search;
use std::collections::HashMap;
use std::sync::Arc; //, Mutex};
use std::thread;
use std::time::SystemTime;

// Iterates on the topmost u64 quadword bytes (8 u8 words x 0-255 bytes(chars))
fn main() {
    let addr_map: HashMap<String, u8> = load::load();

    // Create a new HashMap and wrap it in an Arc and Mutex for shared access
    let map: Arc<HashMap<String, u8>> = Arc::new(addr_map);

    // Iterates over each char between 1..256
    // Char zero iterates on basic u32 range on the lower 64 byte only (basic search)
    // chars1..=8 iterates on the higher u64 byte (profanity algorithm)

    for char_item in 111..=112 {
        let now_init = SystemTime::now();

        // Each thread spawned iterates over one high byte position in u64 byte array u8:32 [8,7,6,5,4,3,2,1]
        // 4th quadword 64 bytes of the private key [u64 ;4]

        let mut handles = vec![];
        for b_pos in 1..=8 {
            let map = Arc::clone(&map);
            let handle = thread::spawn(move || {
                println!("Opened thread for high byte position {b_pos}");
                search::generate_and_check_address_db(b_pos, char_item, &map);
            });
            handles.push(handle);
        }

        // Wait for all threads to finish
        for handle in handles {
            handle.join().unwrap();
        }

        let total_elapsed_time = now_init.elapsed().unwrap().as_millis() / 1000;
        println!("Tread char:{} finished! - Total elapsed time: {:?} ", char_item, total_elapsed_time);
    }
}

// cargo build --release
// ./target/release/van_mar_2023_hashmap_thread_byteorder_new
