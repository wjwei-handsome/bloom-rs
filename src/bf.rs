// 1. we need a bit array: bitvec

use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;
use bitvec::prelude::*;

// define a constant bit array size: 10240
const BIT_ARRAY_SIZE: usize = 10240;

// dine a type: A Hasher array
type HasherArray = Box<[Box<dyn BuildHasher<Hasher = DefaultHasher>>]>;

pub struct BloomFilter<T: Hash + ?Sized> {
    capacity: usize, // Bit array size
    hashers: HasherArray, // Hasher array
    bit_array: BitVec, // Bit array
    // as compiler suggested, we can use PhantomData to make the type T used
    _phantom: PhantomData<T>
}

// impl a default trait for BloomFilter

impl<T: Hash + ?Sized> Default for BloomFilter<T> {
    fn default() -> Self {
        let hash_init_vec: Vec<Box<dyn BuildHasher<Hasher = DefaultHasher>>> = vec![Box::new(RandomState::new())];
        // different RandomState will generate different hashers
        let hash_array = HasherArray::from(hash_init_vec);
        BloomFilter {
            capacity: BIT_ARRAY_SIZE,
            hashers: hash_array,
            bit_array: BitVec::repeat(false, BIT_ARRAY_SIZE),
            _phantom: PhantomData
        }
    }
}

impl<T: Hash + ?Sized> BloomFilter<T> {
    // define a new function
    pub fn new() -> Self {
        Self::default()
    }

    // define with_capacity function
    // pub fn with_capacity(capacity: usize) -> Self {
    //     let hash_init_vec = vec![Box::new(RandomState::new())];
    //     let hash_array = HasherArray::from(hash_init_vec);
    //     BloomFilter {
    //         capacity,
    //         hashers: hash_array,
    //         bit_array: BitVec::repeat(false, capacity),
    //         _phantom: PhantomData
    //     }
    // }

    // define a function: calculate the hash value, get the mod value
    fn calculate_hash(&self, idx: usize, item: &T) -> u64 {
        let mut hasher = self.hashers[idx].build_hasher();
        item.hash(&mut hasher);
        hasher.finish() % (self.capacity as u64)
    }

    // define a function: set bit status
    pub fn set_bit(&mut self, item: &T) {
        for idx in 0..self.hashers.len() {
            let bit_hash_value = self.calculate_hash(idx, item);
            self.bit_array.set(bit_hash_value as usize, true);
        }
    }

    // define a function: get bit status
    pub fn judge_contain(&self, item: &T) -> bool {
        for idx in 0..self.hashers.len() {
            let bit_hash_value = self.calculate_hash(idx, item);
            match self.bit_array.get(bit_hash_value as usize) {
                Some(result) => {
                    if !result { return false; } // if bit offset is false, return false
                },
                _ => return false // if get None, return false
            }
        }
        true
    }
}
