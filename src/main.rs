mod bf;
use bitvec::prelude::*;

fn main() {
    // println!("Hello, world!");
    let test_array = bitarr![u32, Lsb0; 0; 80];
    println!("test_array: {:?}", test_array);
}
