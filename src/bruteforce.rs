use std::ops::Range;
use std::time::Instant;
use chrono::NaiveDateTime;
use num_bigint::BigInt;
use num_traits::Zero;
use crate::arib_rand::AribRandom;
use crate::next_prime::next_prime;
use itertools::Itertools;

pub fn batched_bruteforce(timestamps: impl Iterator<Item=u32>, n: BigInt, deepness: u8, batch_size: u32) -> Option<(BigInt, BigInt, u32)> {
    for batch in &timestamps.chunks(batch_size.try_into().unwrap()) {
        let mut batch = batch.peekable();

        let start_timestamp = *batch.peek().unwrap();
        let end_timestamp = start_timestamp + batch_size;

        let measurement = Instant::now();
        if let Some(result) = bruteforce(batch, n.clone(), deepness) {
            return Some(result);
        }
        let time_elapsed = measurement.elapsed();

        println!("DONE {} -> {}, it took {:?}",
                 NaiveDateTime::from_timestamp_opt(start_timestamp as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S"),
                 NaiveDateTime::from_timestamp_opt(end_timestamp as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S"),
                 time_elapsed
        );
    }
    None
}

pub fn bruteforce(timestamps: impl Iterator<Item=u32>, n: BigInt, deepness: u8) -> Option<(BigInt, BigInt, u32)> {
    let rand_stop = BigInt::from(10).pow(100);
    let mut r = AribRandom::new();
    for t in timestamps {
        r.random_seed_by_timestamp(t);
        for _ in 0..deepness {
            let a = r.random(rand_stop.clone());
            let p = next_prime(a);
            if (&n % &p).is_zero() {
                return Some((&n / &p, p, t));
            }
        }
    }
    None
}