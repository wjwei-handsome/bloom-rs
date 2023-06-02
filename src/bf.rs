// 1. we need a bit array: bitvec

use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{BuildHasher, Hash};
use std::os::fd::BorrowedFd;
use bitvec::prelude::*;

// define a constant bit array size: 10240
const BIT_ARRAY_SIZE: usize = 10240;

// dine a type: A Hasher array
type HasherArray = Box<[Box<dyn BuildHasher<Hasher = DefaultHasher>>]>;

pub struct BloomFilter<T: Hash + ?Sized> {
    capacity: usize, // Bit array size
    hashers: HasherArray, // Hasher array
    bit_array: BitVec // Bit array
}

// impl a default trait for BloomFilter

impl Default for BloomFilter<T> {
    fn default() -> Self {
        let hash_init_vec = vec![Box::new(RandomState::new())];
        // different RandomState will generate different hashers
        let hash_array = HasherArray::from(hash_init_vec);
        BloomFilter {
            capacity: BIT_ARRAY_SIZE,
            hashers: hash_array,
            bit_array: BitVec::repeat(false, BIT_ARRAY_SIZE)
        }
    }
}

impl BloomFilter<T> {
    // define a new function
    pub fn new() -> Self {
        Self::default()
    }

    // difine with_capacity function
    pub fn with_capacity(capacity: usize) -> Self {
        let hash_init_vec = vec![Box::new(RandomState::new())];
        let hash_array = HasherArray::from(hash_init_vec);
        BloomFilter {
            capacity: capacity,
            hashers: hash_array,
            bit_array: BitVec::repeat(false, capacity)
        }
    }
}
