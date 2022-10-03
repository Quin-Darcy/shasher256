use shasher;
use std::env;
use num_bigint::BigUint;

fn main() {
    let args: Vec<String> = env::args().collect();
    let flag: Vec<u8> = args[1].as_bytes().to_vec();
    let data: &str = &args[2];

    if flag.len() < 2 || flag.len() > 2 {
        panic!("Something is wrong with what you input");
    }
    if flag[0] != 45 {
        panic!("Something is wrong with what you input");
    }
    if flag[1] != 102_u8 && flag[1] != 115_u8 {
        panic!("Something is wrong with what you input");
    }

    let hash: BigUint = shasher::get_hash(&flag[1], data); 
    println!("{:0x}", hash);
}
