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
use crate::arib_rand::{AribRandom, AribRandomWindows};
use crate::bruteforce::{batched_bruteforce, bruteforce, BruteforceResult, threaded_bruteforce, threaded_bruteforce_multi_progressbar};
use crate::c_rand::CRandom;
use crate::ranges::{rearrange_ranges, Ranges};

type Timestamp = u32;

fn range(from: &str, to: &str) -> Range<u32> {
    let from = DateTime::<Local>::from_str(&format!("{} +0100", from)).unwrap().timestamp() as u32;
    let to = DateTime::<Local>::from_str(&format!("{} +0100", to)).unwrap().timestamp() as u32;
    from..to
}

fn main() {

    let t = 1703280459u32;
    println!("Scroll of truth said: {}", NaiveDateTime::from_timestamp_opt(t as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S"));

    let measure = Instant::now();

    // let timerange = range("2023-12-22 00:00:00", "2023-12-22 23:59:59");
    let target = BigInt::from_str("175_57269_22656_47610_58334_46854_73969_47771_69413_44027_01823_44030_29905_25519_46713_72215_73916_44501_30281_22708_30868_65448_05279_78456_81918_72276_61288_69080_53876_26781_86282_40821_55218_93311_21256_41224_29306_09557_00554_71931").unwrap();

    let timerange = (t - 120)..(t + 120);

    let deepness = 1;
    let num_threads = 16;

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