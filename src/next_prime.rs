use num_bigint::{BigInt, ToBigInt};
use num_traits::identities::*;
use num_bigint::RandBigInt;
use rand;


fn miller_test(init: (BigInt, usize), n: &BigInt) -> bool {
    let (d, its) = init;
    let mut rng = rand::thread_rng(); // TODO what is the performance cost of this call?

    // is_prime() checks for n < 4
    let a = 2 + rng.gen_bigint_range(&1.into(), &(n - BigInt::from(4)));

    let mut x = BigInt::modpow(&a, &d, &n);

    if x == 1.into() || x == n - 1 {
        return true;
    }

    for _ in 0..its {
        x = x.modpow(&2.into(), n);
        if x == n - 1 {
            return true;
        }
    }
    false
}

pub fn is_prime(num: &BigInt) -> bool {
    let one: BigInt = One::one();
    if num <= &one || num == &4.into() {
        return false;
    }
    if num <= &BigInt::from(3) {
        return true;
    }

    let mut d: BigInt = num - 1;
    let mut its = 0;
    while (&d % BigInt::from(2)).is_zero() {
        d /= BigInt::from(2);
        its += 1;
    }

    for _ in 0..16 {
        if miller_test((d.clone(), its), num) == false {
            return false;
        }
    }
    true
}

pub fn next_prime(n: BigInt) -> BigInt {
    let two = BigInt::from(2);
    if n == two {
        return n;
    }
    let mut result = n;
    if (&result % &two).is_zero() {
        result += 1;
    }
    while !is_prime(&result) {
        result += &two;
    }
    return result;
}
