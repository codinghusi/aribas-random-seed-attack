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

    let t = 1703283510;
    println!("Scroll of truth said: {}", NaiveDateTime::from_timestamp_opt(t as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S"));

    let measure = Instant::now();

    // let timerange = range("2023-12-22 00:00:00", "2023-12-22 23:59:59");
    let target = BigInt::from_str("47271_59574_59763_68415_33447_92676_65329_14158_62478_16179_57909_25741_92028_51745_92083_15950_61430_62446_30345_81188_34180_82451_20958_04341_65176_64490_85546_61901_69479_26730_92650_86850_74318_68645_43716_46484_80900_34209_42107_70977").unwrap();

    let deepness = 2;
    let num_threads = 16;

    let timerange = t-1000..(t + 10000);
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