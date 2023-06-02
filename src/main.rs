use bitvec::macros::internal::funty::Fundamental;

mod bf;

fn main() {
    // println!("Hello, world!");
    let mut bf = bf::BloomFilter::new();

    let test_continuous_nums = 0..10000;
    let positive_nums = 5000..10000;
    let negative_nums = 10000..15000;

    let test_continuous_count = test_continuous_nums.clone().count();
    let positive_count = positive_nums.clone().count();
    let negative_count = negative_nums.clone().count();


    // set bit in bloom filter of 10000 continuous numbers
    println!("Start to set bit in bloom filter of {} continuous numbers", test_continuous_count);
    for i in test_continuous_nums {
        bf.set_bit(&i.to_string());
    }

    println!("Start to judge if {} positive numbers are in bloom filter", positive_count);
    let mut true_positive = 0;
    for i in positive_nums {
        if bf.judge_contain(&i.to_string()) {
            true_positive += 1;
        }
    }
    println!("true-positive-rate: {}/{} = {}", true_positive, positive_count, true_positive.as_f64()/ positive_count.as_f64());


    println!("Start to judge if {} negative numbers are in bloom filter", negative_count);
    let mut false_positive = 0;
    for i in negative_nums {
        if bf.judge_contain(&i.to_string()) {
            false_positive += 1;
        }
    }
    println!("false-positive-rate: {}/{} = {}", false_positive, negative_count, false_positive.as_f64()/ negative_count.as_f64());


    // FP rate = (1-(1-1/m)^nk)^k

    let m = 10240f64; // bit array size
    let n = test_continuous_count.as_f64(); // insert number
    let k = 1f64; // hasher number
    let fp_rate = (1.0 - (1.0 - 1.0/m).powf(n as f64 * k as f64)).powf(k as f64);
    println!("Theoretical false-positive-rate: {}", fp_rate);
}
