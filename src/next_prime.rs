use num_bigint::BigInt;
use num_traits::identities::*;
use num_bigint::RandBigInt;
use rand;


fn miller_test(mut d: BigInt, n: &BigInt) -> bool {
    let one: BigInt = One::one();
    let two: BigInt = &one + &one;
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let mut random_num = one.clone();
    if n != &BigInt::from(5) {
         random_num = rng.gen_bigint_range(&one, &(n - BigInt::from(4)));
    }
    let a = BigInt::from(2) + random_num;

    let mut x = BigInt::modpow(&a, &d, &n);

    if x == one || x == n - &one {
        return true;
    }

    while d != n - &one {
        x = (&x * &x) % n;
        d *= &two;

        if x == one {
            return false;
        }
        if x == n - &one {
            return true;
        }
    }

    false
}

pub fn is_prime(num: &BigInt) -> bool {
    let one: BigInt = One::one();
    if num <= &one || num == &BigInt::from(4) {
        return false;
    }
    if num <= &BigInt::from(3) {
        return true;
    }

    let mut d = num - &one;
    while &d % 2 == Zero::zero() {
        d /= BigInt::from(2);
    }

    for _ in 0..16 {
        if miller_test(d.clone(), num) == false {
            return false;
        }
    }
    true
}

pub fn next_prime(n:&BigInt) -> BigInt {
    let one: BigInt = One::one();
    let zero: BigInt = Zero::zero();
    let two: BigInt = &one + &one;
    let mut result:BigInt = n.clone();
    if n % 2 == zero {
        result = n + &one;
    }
    while !is_prime(&result) {
        result = result + &two;
    }
    return result;

}
