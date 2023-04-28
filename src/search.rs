use secp256k1::{Error, PublicKey, Secp256k1, SecretKey};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use tiny_keccak::{Hasher, Keccak};

fn add_points(point1: PublicKey, point2: PublicKey) -> Result<PublicKey, Error> {
    let _secp256k1 = Secp256k1::new();
    let point_sum = point1.combine(&point2)?;
    Ok(point_sum)
}

pub fn generate_and_check_address_db(b_pos: u8, char: u8, addr_map: &HashMap<String, u8>) {
    // Initialize Secp256k1 object
    let secp = Secp256k1::new();

    // Calculate point generator
    println!("-------------------------------------");
    let point_g = "0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
    let constant_g = PublicKey::from_slice(&hex::decode(point_g).unwrap()).unwrap();
    println!("Constant   G    : {:?}", constant_g);

    // ---> Here you set the for-loop range <---
    let ini: u64 = 1; // 1;  // 3_000_000_000; //1;
    let end: u64 = 4_294_967_296;

    // Calculate public key from the initial number
    let seed: [u8; 32] = get_initial_seed(b_pos, char, ini);

    let secret_key = SecretKey::from_slice(&seed).unwrap();
    let start_public_key = PublicKey::from_secret_key(&secp, &secret_key);

    println!("----------------------------------------------");
    println!("b_pos: {b_pos} - char: {char}");
    println!("seed: {:?}", seed);
    println!("start_public_key: {:?}", start_public_key);
    println!("----------------------------------------------");

    // Calculate next public key (i+1) - Sum public key to G (public_key_G)
    let mut public_key_iter = start_public_key.clone();

    let mut now = SystemTime::now();

    for iter in ini + 1..=end {
        // Sums the constant G to the last public key (public_key_iter) and creates a new Public Key object instance
        let new_public_key_iter = add_points(public_key_iter, constant_g).unwrap();

        // Clones the public_key instance for safety
        public_key_iter = new_public_key_iter.clone();
        // println!("pk              : {:?}", public_key_iter);

        // Extracts the public key uncompressed from the Public Key Object instance
        let public_key: [u8; 65] = public_key_iter.serialize_uncompressed();
        // println!("public_key 65   : {:?}", public_key);

        // Hashes the public key to calculate address
        let pub_key_hashed = keccak256(&public_key[1..]);
        // println!("pk_hashed       : {:?}", pub_key_hashed);

        // Extracts the address from the last 20 bytes of the hashed pub key
        let mut bytes = [0u8; 20];
        bytes.copy_from_slice(&pub_key_hashed[12..]);
        // println!("bytes_to_hash    :  {:?}",bytes// );

        // Converts address to String (for use in a HashMap table search)
        // Takes out "0x" string
        // Get only the first 20 bytes of the hex address to speed HashMap search (only 10 bytes of the [u8] array)

        let address_to_print = hex::encode(&bytes);
        let address_to_seek = hex::encode(&bytes[0..10]);

        // Verify (query) if the address is recorded in the HashMap
        let result = addr_map.get(&address_to_seek);
        // println!("address_str  : {i} - {address}");

        // println!("b_pos: {b_pos} - char : {char} - iter: {iter} - address: {address_to_print}");

        match result {
            Some(_balance) => {
                publish_found_results(b_pos, char, iter, address_to_print, public_key);
            }
            None => {
                // println!("Não encontrado: address_str: {address_str} - count {counter}");
                if iter % 1_000_000_000 == 0 {
                    let total_time = now.elapsed().unwrap().as_millis() / 1000;
                    publish_not_found_results(b_pos, char, iter, address_to_print, total_time);
                    // reset timer
                    now = SystemTime::now();
                }
            }
        }
    }
}

fn publish_found_results(b_pos: u8, char: u8, iter: u64, address: String, public_key: [u8; 65]) {
    println!("================================================================================================");
    println!("Found!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    println!("char      :{char}  - bytepos:{b_pos} ");
    println!("iteration : {iter} - address: {address}");
    println!("public_key: {:?}", public_key);
    println!("================================================================================================");

    let mut output = String::new();
    fmt::write(
        &mut output,
        format_args!("============================== Found ============================\n char:{char} - bytepos:{b_pos} - iter:{iter} - {address:?}\npublic_key: {public_key:?}\n=================================================================\n"),
    )
    .expect("Error occurred while trying to write in String");
    write_results(char, &output);
}

fn publish_not_found_results(b_pos: u8, char: u8, iter: u64, address: String, total_time: u128) {
    println!("char:{char} - bytepos:{b_pos} - iter:{iter}-{address} - {total_time} s");

    let mut output = String::new();
    fmt::write(&mut output, format_args!("char:{char} - bytepos:{b_pos} - iter:{iter} - {address}\n")).expect("Error occurred while trying to write in String");
    write_results(char, &output);
}

fn write_results(b_pos: u8, output: &str) {
    let path_str = format!("char{b_pos}.txt");
    let path = Path::new(&path_str);

    if !path.exists() {
        let _ = File::create(&path_str).unwrap();
    }

    let mut file_ref = OpenOptions::new().append(true).open(path).expect("Unable to open file");

    file_ref.write_all(output.as_bytes()).expect("Atenção: gravação no arquivo result.txt falhou!");
}

pub fn keccak256<T: AsRef<[u8]>>(bytes: T) -> [u8; 32] {
    let mut output = [0u8; 32];

    let mut hasher = Keccak::v256();
    hasher.update(bytes.as_ref());
    hasher.finalize(&mut output);
    output
}

fn get_initial_seed(b_pos: u8, char: u8, iter: u64) -> [u8; 32] {
    let mut _seed = [0_u8; 32];
    match b_pos {
        1 => {
            _seed = [
                0, 0, 0, 0, 0, 0, 0, char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        }
        2 => {
            _seed = [
                0, 0, 0, 0, 0, 0, char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        }
        3 => {
            _seed = [
                0, 0, 0, 0, 0, char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        }
        4 => {
            _seed = [
                0, 0, 0, 0, char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        }
        5 => {
            _seed = [
                0, 0, 0, char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        }
        6 => {
            _seed = [
                0, 0, char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        }
        7 => {
            _seed = [
                0, char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        }
        8 => {
            _seed = [
                char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        }
        _ => _seed = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    }

    // high_byte_array
    let l: [u8; 8] = iter.to_be_bytes();

    println!("---------------------------------------------");
    println!("iter          : {iter}");
    println!("iter_be_bytes : {l:?}");

    // Track the iter number
    // insert the bytes of iter u64 - in the low bytes of result
    _seed[24] = l[0];
    _seed[25] = l[1];
    _seed[26] = l[2];
    _seed[27] = l[3];
    _seed[28] = l[4];
    _seed[29] = l[5];
    _seed[30] = l[6];
    _seed[31] = l[7];

    _seed
}

// Used when Redis is used as db
// let client = redis::Client::open("redis://127.0.0.1/").unwrap();
// let mut con = client.get_connection().unwrap();

// address.insert_str(0, "0x");
// println!("address         :                       {:?}", address);

// // Verify (query) if the address is recorded in the Redis database
// let result = con.get(&address).unwrap_or(0i32);
// pub fn is_db_online() {
//     if !is_redis_online() {
//         println!("===========================================================");
//         println!("ATTENTION! Redis db is offline! program aborted!");
//         println!("===========================================================");
//         std::process::exit(1);
//     };
// }

// fn is_redis_online() -> bool {
//     // 0 - Set performance parameters
//     let mut test_array: [String; 18] = Default::default();
//     test_array[0] = String::from("0x4cceba2d7d2b4fdce4304d3e09a1fea9fbeb1528");
//     test_array[1] = String::from("0xd41c057fd1c78805aac12b0a94a405c0461a6fbb");
//     test_array[2] = String::from("0x8735015837bd10e05d9cf5ea43a2486bf4be156f");
//     test_array[3] = String::from("0x252dae0a4b9d9b80f504f6418acd2d364c0c59cd");
//     test_array[4] = String::from("0x3bc8287f1d872df4217283b7920d363f13cf39d8");
//     test_array[5] = String::from("0x883d01eae6eaac077e126ddb32cd53550966ed76");
//     test_array[6] = String::from("0x7aabc7915df92a85e199dbb4b1d21e637e1a90a2");
//     test_array[7] = String::from("0x3183ade12d4946475d9740dd132c4ca1061b9eea");
//     test_array[8] = String::from("0xcbda2c55da6e6441a25bfafc9f8c614c9c7fca3a");
//     test_array[9] = String::from("0x913855868d0f21d143c332dc07b8b249451b4f5f");
//     test_array[10] = String::from("0xaf645ab37ef0306bf5a09eb4c7499a3de8750358");
//     test_array[11] = String::from("0x03b90665077b47c675ea9389ef197a680c61cb69");
//     test_array[12] = String::from("0x89c62041aca94c231ebbd0b8a796048d9b9e9d74");
//     test_array[13] = String::from("0x89c62041aca94c231ebbd0b8a796048d9b9e9d74");
//     test_array[14] = String::from("0x5a5bf61e1930501bb55d1a92e1303e384d01d1b1");
//     test_array[15] = String::from("0x3742c079c0626266f4b064915573b96fc0d14551");
//     test_array[16] = String::from("0xc1a9bc22cc31ab64a78421befa10c359a1417ce3");
//     test_array[17] = String::from("0x0a4c79ce84202b03e95b7a692e5d728d83c44c76");

//     let mut is_redis_online = true;

//     // 1 - Set database handlers
//     let client2 = redis::Client::open("redis://127.0.0.1/").unwrap();
//     let mut con2 = client2.get_connection().unwrap();

//     // 2 -Read input eth_addresses array item by item
//     for item in test_array {
//         let address = item.to_owned();
//         // // Generate Signing Key
//         // let signing_key = SigningKey::from_bytes(&seed_as_bytes).unwrap();
//         // //println!("signing_key  : {:?}", signing_key);
//         let result = con2.get(&address).unwrap_or(0i32);
//         println!("Redis is online {:?} - {:?} ", address, result);

//         match result {
//             1 => {}
//             0 => {
//                 is_redis_online = false;
//             }
//             _ => {
//                 is_redis_online = false;
//             }
//         }
//         // println!("{:?} - {} --> Found!", result, address)
//     }

//     return is_redis_online;
// }
