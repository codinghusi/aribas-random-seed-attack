#![allow(dead_code)]
#![allow(unused_imports)]

mod c_rand;
mod arib_rand;
mod next_prime;
mod tests;
mod bruteforce;
mod ranges;

use std::ops::Range;
use std::str::FromStr;
use std::time::Instant;
use num_bigint::BigInt;
use next_prime::next_prime;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use crate::arib_rand::AribasRandom;
use crate::bruteforce::{batched_bruteforce, bruteforce, BruteforceResult, threaded_bruteforce, threaded_bruteforce_multi_progressbar};
use crate::ranges::{rearrange_ranges, Ranges};

type Timestamp = u32;

fn range(from: &str, to: &str) -> Range<u32> {
    let from = DateTime::<Local>::from_str(&format!("{} +0100", from)).unwrap().timestamp() as u32;
    let to = DateTime::<Local>::from_str(&format!("{} +0100", to)).unwrap().timestamp() as u32;
    from..to
}

fn main() {

    // let target = 408;
    // let range = 1703281420-1000..1703281420+1000;
    // for t in range {
    //     let mut r = AribRandomWindows::new();
    //     r.random_seed_by_timestamp(t);
    //     if r.random(BigInt::from(1000)) == BigInt::from(target) {
    //         println!("found it");
    //         break;
    //     }
    // }

    let t = 1703281694;
    println!("Scroll of truth said: {}", NaiveDateTime::from_timestamp_opt(t as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S"));

    let measure = Instant::now();

    // let timerange = range("2023-12-22 00:00:00", "2023-12-22 23:59:59");
    let target = BigInt::from_str("3684_61746_05577_50683_10709_42640_93223_79572_04755_58317_08314_90649_74909_67418_72514_69967_00182_03247_29099_27120_99919_17106_16898_35791_94785_14437_34333_99377_35415_52847_18410_88701_77332_76251_62950_44103_76810_31659_00986_29913").unwrap();

    let deepness = 1;
    let num_threads = 16;

    let timerange = t+10000..(t + 10000*2);
    let len = timerange.len() as u32;

    match threaded_bruteforce(timerange, target, deepness, num_threads, len) {
        Some(BruteforceResult { p, q, timestamp }) => {
            println!("Found it!");
            println!("p = {}", p);
            println!("q = {}", q);
            println!("Aribas was opened at {}", NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S"));
        }
        None => println!("Unfortunately, bruteforce didn't work :/")
    }

    println!("took at whole: {:?}", measure.elapsed());
}