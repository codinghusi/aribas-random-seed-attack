use std::ops::Range;
use std::time::Instant;
use chrono::NaiveDateTime;
use core::time::Duration;
use num_bigint::BigInt;
use num_traits::Zero;
use crate::arib_rand::AribRandom;
use crate::next_prime::next_prime;
use itertools::Itertools;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

enum Status {
    Checked {
        timestamp_from: u32,
        timestamp_to: u32,
        time_elapsed: Duration
    },
    Found(BruteforceResult)
}

pub struct BruteforceResult {
    pub p: BigInt,
    pub q: BigInt,
    pub timestamp: u32
}

pub fn threaded_bruteforce(timestamps: impl Iterator<Item=u32>, n: BigInt, deepness: u8, thread_count: usize, total_seconds: u32) -> Option<BruteforceResult> {
    let batch_size = 60;
    let pool = ThreadPool::new(thread_count);

    let (tx, rx) = channel::<Status>();

    for batch in &timestamps.chunks(batch_size.try_into().unwrap()) {
        let tx = tx.clone();
        let batch = batch.collect::<Vec<_>>();
        let n = n.clone();
        pool.execute(move|| {
            let start_timestamp = batch[0];
            let end_timestamp = start_timestamp + batch_size;

            let measurement = Instant::now();
            if let Some(result) = bruteforce(batch.into_iter(), n.clone(), deepness) {
                tx.send(Status::Found(result)).unwrap();
                return;
            }
            let time_elapsed = measurement.elapsed();

            tx.send(Status::Checked {
                timestamp_from: start_timestamp,
                timestamp_to: end_timestamp,
                time_elapsed
            }).unwrap();
        })
    }

    for status in rx.iter().take(total_seconds.try_into().unwrap()) {
        match status {
            Status::Checked { timestamp_from, timestamp_to, time_elapsed} => {
                println!("DONE {} -> {}, it took {:?}",
                         NaiveDateTime::from_timestamp_opt(timestamp_from as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S"),
                         NaiveDateTime::from_timestamp_opt(timestamp_to as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S"),
                         time_elapsed
                );
            }
            Status::Found(result) => return Some(result)
        }
    }

    None
}

pub fn batched_bruteforce(timestamps: impl Iterator<Item=u32>, n: BigInt, deepness: u8, batch_size: u32) -> Option<BruteforceResult> {
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

pub fn bruteforce(timestamps: impl Iterator<Item=u32>, n: BigInt, deepness: u8) -> Option<BruteforceResult> {
    let rand_stop = BigInt::from(10).pow(100);
    let mut r = AribRandom::new();
    for t in timestamps {
        r.random_seed_by_timestamp(t);
        for _ in 0..deepness {
            let a = r.random(rand_stop.clone());
            let p = next_prime(a);
            if (&n % &p).is_zero() {
                return Some(BruteforceResult {
                    q: n / &p,
                    p,
                    timestamp: t
                });
            }
        }
    }
    None
}