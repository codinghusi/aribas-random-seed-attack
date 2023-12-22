#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use num_bigint::BigInt;
    use crate::arib_rand::AribRandom;

    #[test]
    fn seed_1() {
        let mut r = AribRandom::new();
        assert_eq!(r.random_seed(1),
                   28147_49767_10657u64.into());
        assert_eq!(r.random(10.into()),
                   6.into());
        assert_eq!(r.random(100.into()),
                   78.into());
        assert_eq!(r.random(BigInt::from(2).pow(17)),
                   119744.into());
        assert_eq!(r.random(BigInt::from(2).pow(33)),
                   241908038.into());
        assert_eq!(r.random(BigInt::from(2).pow(65)),
                   15385_00179_53388_62151u64.into());
        assert_eq!(r.random(BigInt::from(10).pow(100)),
                   BigInt::from_str("74727_96161_92728_01821_95922_60889_43807_47203_12396_32823_52618_70812_31250_41240_21994_62754_64548_54406_76188_11538").unwrap());
    }
}