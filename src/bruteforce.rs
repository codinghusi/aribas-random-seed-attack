use std::ops::Range;
use std::time::Instant;
use chrono::NaiveDateTime;
use core::time::Duration;
use std::collections::{HashMap, VecDeque};
use std::fmt::Write;
use std::rc::Rc;
use num_bigint::BigInt;
use num_traits::Zero;
use crate::arib_rand::{AribRandom, AribRandomWindows};
use crate::next_prime::next_prime;
use itertools::Itertools;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use crate::ranges::{ChunkedRanges, OneChunk, Ranges};

enum Status {
    Done {
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

fn get_hour(timestamp: u32) -> u32 {
    timestamp - timestamp % (60*60)
}

pub fn threaded_bruteforce_with_progressbar(timestamps: impl Iterator<Item=u32>, n: BigInt, deepness: u8, thread_count: usize, total_seconds: u32) -> Option<BruteforceResult> {
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

            tx.send(Status::Done {
                timestamp_from: start_timestamp,
                timestamp_to: end_timestamp,
                time_elapsed
            }).unwrap();
        })
    }


    let pb = ProgressBar::new((total_seconds / batch_size) as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    pb.set_position(0);

    for status in rx.iter().take(total_seconds.try_into().unwrap()) {
        match status {
            Status::Done { timestamp_from: _, timestamp_to: _, time_elapsed: _} => {
                pb.inc(1);
            }
            Status::Found(result) => return Some(result),
        }
    }

    pb.finish_with_message("done");

    None
}

pub fn threaded_bruteforce_multi_progressbar(timestamps: Ranges, n: BigInt, deepness: u8, thread_count: usize, total_seconds: u32) -> Option<BruteforceResult> {
    let batch_size = 60;
    let pool = ThreadPool::new(thread_count);

    let (tx, rx) = channel::<Status>();
    // let m = MultiProgress::new();
    // let style = ProgressStyle::with_template(
    //     "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    // ).unwrap();

    for batch in ChunkedRanges::new(timestamps, 60) {
        println!("batch: {:?}", batch);
        let tx = tx.clone();
        let n = n.clone();
        let batch = OneChunk::new(batch);
        pool.execute(move|| {
            let start_timestamp = batch.first;
            let end_timestamp = start_timestamp + batch_size;

            let measurement = Instant::now();
            if let Some(result) = bruteforce(batch, n.clone(), deepness) {
                tx.send(Status::Found(result)).unwrap();
                return;
            }
            let time_elapsed = measurement.elapsed();

            tx.send(Status::Done {
                timestamp_from: start_timestamp,
                timestamp_to: end_timestamp,
                time_elapsed
            }).unwrap();
        })
    }

    for status in rx.iter().take(total_seconds.try_into().unwrap()) {
        match status {
            Status::Done { timestamp_from, timestamp_to, time_elapsed} => {
                println!("DONE {} -> {}, it took {:?}",
                         NaiveDateTime::from_timestamp_opt(timestamp_from as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S"),
                         NaiveDateTime::from_timestamp_opt(timestamp_to as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S"),
                         time_elapsed
                );
            }
            Status::Found(result) => return Some(result),
        }
    }

    None
}

pub fn threaded_bruteforce(timestamps: impl Iterator<Item=u32>, n: BigInt, deepness: u8, thread_count: usize, total_seconds: u32) -> Option<BruteforceResult> {
    let batch_size = 120;
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

            tx.send(Status::Done {
                timestamp_from: start_timestamp,
                timestamp_to: end_timestamp,
                time_elapsed
            }).unwrap();
        })
    }

    for status in rx.iter().take((total_seconds / batch_size) as usize) {
        match status {
            Status::Done { timestamp_from, timestamp_to, time_elapsed} => {
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
    let mut r = AribRandomWindows::new();
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