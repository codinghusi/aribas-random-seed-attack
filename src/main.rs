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
use std::iter::Extend;

type Timestamp = u32;

fn range(from: &str, to: &str) -> Range<u32> {
    let from = DateTime::<Local>::from_str(&format!("{} +0100", from)).unwrap().timestamp() as u32;
    let to = DateTime::<Local>::from_str(&format!("{} +0100", to)).unwrap().timestamp() as u32;
    from..to
}

fn main() {
    let measure = Instant::now();

    let target = BigInt::from_str("32115920099212569105106779097640231161112770207070059646211939683635728962265075652991614878272272454539611555745310747733885970866333511478662533604607053662543710242659089271255121936433439154585717").unwrap();

    let deepness = 2;
    let num_threads = 16;

    let timerange = range("2018-11-20 00:00:00", "2018-12-04 00:00:00")
                                   .chain(range("2019-11-20 00:00:00", "2019-12-04 00:00:00"))
                                   .chain(range("2020-11-20 00:00:00", "2020-12-04 00:00:00"))
                                   .chain(range("2021-11-20 00:00:00", "2021-12-04 00:00:00"))
                                   .chain(range("2022-11-20 00:00:00", "2022-12-04 00:00:00"));
    let len = (range("2018-11-20 00:00:00", "2018-12-04 00:00:00").len() as u32) * 5;

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