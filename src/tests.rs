#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use num_bigint::BigInt;
    use crate::arib_rand::{AribasRandom};
    use crate::c_rand::{CRandomLinux, CRandomWindows};
    use crate::next_prime::next_prime;

    #[test]
    fn seed_1() {
        let mut r = AribasRandom::new_linux();
        assert_eq!(r.random_seed(1),
                   28147_49767_10657u64);
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

    #[test]
    fn seed_2() {
        let mut r = AribasRandom::new_linux();
        assert_eq!(r.random_seed(123456789),
                   28147_51001_67445u64);
        assert_eq!(r.random(1234.into()),
                   1108.into());
        assert_eq!(r.random(123456789.into()),
                   39308462.into());
        assert_eq!(r.random(987654321.into()),
                   548909087.into());
    }

    #[test]
    fn time_seed() {
        let mut r = AribasRandom::new_linux();
        r.random_seed_by_timestamp(1703227980);
        assert_eq!(r.get_current_seed(), 34343_13636_56552u64);
    }

    #[test]
    fn next_prime_1() {
        let n = BigInt::from(10).pow(100);
        assert_eq!(next_prime(n.clone()), n + 267);
    }

    #[test]
    fn seed_windows_1() {
        let mut r = AribasRandom::new_windows();
        let timestamp = 1703276746;
        let seed = r.random_seed_by_timestamp(timestamp);
        assert_eq!(seed, 44789_13194_87473u64)
    }

    #[test]
    fn crand_windows() {
        let mut r = CRandomWindows::new();
        r.srand(123);
        assert_eq!(r.rand(), 440);
    }

    #[test]
    fn crand_linux() {
        let mut r = CRandomLinux::new();
        r.srand(123);
        assert_eq!(r.rand(), 128959393);
    }
}