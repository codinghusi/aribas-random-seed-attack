#![allow(dead_code)]

mod c_rand;
mod arib_rand;
mod tests;
use num_bigint::BigInt;
use crate::arib_rand::AribRandom;

fn main() {
    // let middle = 1703179901;
    // let r = 10;
    // let start: u32 = middle - r;
    // let end: u32 = middle + r;
    // let target: BigInt = BigInt::from_str(&String::from("2857630803201393715472230059466985967968662716706032079401208133638055838905612545189650202214329263")).unwrap();
    // // let target: u64 = 40755_41527_08641;
    // for t in start..=end {
    //     let mut r = AribRandom::init_timestamp(t);
    //     let n = r.random(BigInt::from(10).pow(100));
    //     println!("rand = {}", n);
    //
    //     if n == target {
    //         println!("JAAAA");
    //         break;
    //     }
    //
    //
    //
    //     // println!("random: {}", r.random(BigInt::from(1000000)));
    // }

    let mut r = AribRandom::new();
    r.random_seed(100);
    println!("{}", r.random(BigInt::from(10_000_000)));

    // println!("timestamp {}", timestamp());

    // let mut r = AribRandom::init_timestamp(1);
    // println!();
    // println!("seed: {}, random: {}", r.current_seed(), r.random(BigInt::from(100)));
    // println!("wanted");
    // println!("seed: {}, random: {}", 40999_47846_45464u64, 78);
    //
    // let mut r = AribRandom::init_timestamp(1);
    // println!();
    // println!("seed: {}, random: {}", r.current_seed(), r.random(BigInt::from(10).pow(100)));
    // // println!("seed: {}, random: {}", r.current_seed(), r.random(BigInt::from(10).pow(100)));
    // println!("wanted");
    // println!("seed: {}, random: {}",
    //          40999_47846_45464u64,
    //          BigInt::from_str("55680_30889_00205_84074_43538_50309_90444_35182_18816_18299_61764_60818_75813_41698_13965_17677_09884_18351_14710_42350").unwrap(),
    // );
    // println!("seed: {}, random: {}",
    //          38684_29670_31011u64,
    //          BigInt::from_str("31573_80640_55592_34913_44114_07299_61249_85945_47460_54931_62321_14100_7866_45777_65235_79385_73359_38392_66314_24370").unwrap(),
    // );

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