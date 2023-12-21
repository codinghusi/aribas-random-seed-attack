#![allow(dead_code)]

mod c_rand;
mod arib_rand;

use num_bigint::BigInt;
use crate::arib_rand::AribRandom;
// use rand::Random;
// use crate::rand::{rand, srand, timestamp};
use crate::c_rand::CRandom;

fn main() {
    // let start: u32 = 1703088834 - 60;
    // let end: u32 = 1703088834 + 60;
    // let target: u64 = 54543_26706_77402;
    // for t in start..=end {
    //     let mut r = Random::init_timestamp(t);
    //     if r.current_seed() == target {
    //         println!("done!!");
    //         break;
    //     }
    //
    //     println!("t = {}, seed = {}", t, r.current_seed());
    //
    //     // println!("random: {}", r.random(BigInt::from(1000000)));
    // }

    // println!("timestamp {}", timestamp());

    // let mut r = AribRandom::init_timestamp(1);
    // println!("seed: {}, random: {}", r.current_seed(), r.random(BigInt::from(100)));
    // println!("wanted");
    // println!("seed: {}, random: {}", 40999_47846_45464u64, 78);

    let mut r = CRandom::new();
    r.srand(1);
    println!("{} {}", r.rand(), r.rand());

    /*
    rr = 1 0 54156 0
    rr = 1 0 17767 0
    rr = 1 15414 56540 34889
    rr = 1 15414 56540 17767
    rr = 1 29923 22911 41304
    rr = 1 29923 22911 41304
    */

}