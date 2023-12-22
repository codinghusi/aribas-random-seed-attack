#![allow(dead_code)]
#![allow(unused_imports)]

mod c_rand;
mod arib_rand;
mod next_prime;
mod tests;
mod bruteforce;

use std::str::FromStr;
use num_bigint::BigInt;
use next_prime::next_prime;
use chrono::{NaiveDate, NaiveDateTime};
use crate::bruteforce::{batched_bruteforce, bruteforce, BruteforceResult, threaded_bruteforce};


fn main() {
    println!("Scroll of truth said: {}", NaiveDateTime::from_timestamp_opt(1703238264, 0).unwrap().format("%Y-%m-%d %H:%M:%S"));

    let target = BigInt::from_str("1303_36591_76531_11614_30998_87393_66568_69224_49903_84161_10639_32154_64486_84477_66119_63697_36408_67843_80322_29457_34917_09809_85625_81842_33078_45614_11101_73264_39086_46303_90409_28881_21320_72807_73242_14173_06116_88125_03667_22827").unwrap();

    let date = NaiveDate::from_ymd_opt(2023, 12, 22).unwrap();
    let from = date.and_hms_opt(9, 0, 0).unwrap().timestamp() as u32;
    let to = date.and_hms_opt(10, 0, 0).unwrap().timestamp() as u32;

    let deepness = 1;
    let num_threads = 8;

    match threaded_bruteforce(from..to, target, deepness, num_threads, to - from) {
        Some(BruteforceResult { p, q, timestamp }) => {
            println!("Found it!");
            println!("p = {}", p);
            println!("q = {}", q);
            println!("Aribas was opened at {}", NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S"));
        }
        None => println!("Unfortunately, bruteforce didn't work :/")
    }
}