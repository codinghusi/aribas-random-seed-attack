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
use crate::bruteforce::{batched_bruteforce, bruteforce, BruteforceResult, threaded_bruteforce, threaded_bruteforce_fast, threaded_bruteforce_multi_progressbar};
use crate::ranges::{rearrange_ranges, Ranges};
use std::iter::Extend;

type Timestamp = u32;

fn range(from: &str, to: &str) -> Range<u32> {
    let from = DateTime::<Local>::from_str(&format!("{} +0100", from)).unwrap().timestamp() as u32;
    let to = DateTime::<Local>::from_str(&format!("{} +0100", to)).unwrap().timestamp() as u32;
    from..to
}

fn main() {
    let measure = Instant::now();

    let n = BigInt::from_str("32115920099212569105106779097640231161112770207070059646211939683635728962265075652991614878272272454539611555745310747733885970866333511478662533604607053662543710242659089271255121936433439154585717").unwrap();

    let num_threads = 16;

    match threaded_bruteforce_fast(n, num_threads) {
        Some(BruteforceResult { p, q, timestamp: _ }) => {
            println!("Found it!");
            println!("p = {}", p);
            println!("q = {}", q);
        }
        None => println!("Unfortunately, bruteforce didn't work :/")
    }

    println!("took at whole: {:?}", measure.elapsed());
}