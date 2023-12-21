#![allow(dead_code)]

mod c_rand;
mod arib_rand;

use std::str::FromStr;
use num_bigint::BigInt;
use crate::arib_rand::AribRandom;

fn main() {
    let start: u32 = 1703179901 - 60;
    let end: u32 = 1703179901 + 60;
    let target: BigInt = BigInt::from_str(&String::from("2857630803201393715472230059466985967968662716706032079401208133638055838905612545189650202214329263")).unwrap();
    for t in start..=end {
        let mut r = AribRandom::init_timestamp(t);
        let n = r.random(BigInt::from(10).pow(100));

        println!("t = {}, seed = {}", t, r.current_seed());

        if n == target {
            println!("JAAAA");
            break;
        }



        // println!("random: {}", r.random(BigInt::from(1000000)));
    }

    // println!("timestamp {}", timestamp());

    // let mut r = AribRandom::init_timestamp(1);
    // println!();
    // println!("seed: {}, random: {}", r.current_seed(), r.random(BigInt::from(100)));
    // println!("wanted");
    // println!("seed: {}, random: {}", 40999_47846_45464u64, 78);
    //
    // let mut r = AribRandom::init_timestamp(1);
    // println!();
    // println!("seed: {}, random: {}", r.current_seed(), r.random(BigInt::from(70000)));
    // println!("wanted");
    // println!("seed: {}, random: {}", 40999_47846_45464u64, 45422);

    /*
    ==> random_seed2(1).
    rr = 1 0 17767 0
    rr = 1 15414 56540 34889
    rr = 1 15414 56540 17767
    rr = 1 29923 22911 41304
    rr = 1 29923 22911 41304
    -: 40999_47846_45464
    ==> random(70000).
    rr = 1 37329 12078 39393
    -: 45422
    */

}