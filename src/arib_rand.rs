use std::time::SystemTime;
use num_bigint::BigInt;
use num_traits::identities::Zero;
use crate::c_rand::CRandom;


fn time() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub fn sysrand() -> u32 {
    sysrand_timestamp(time() as u32)
}

pub fn sysrand_timestamp(timestamp: u32) -> u32 {
    let mut r = CRandom::new();
    r.srand(timestamp);
    r.rand()
}

type RandomSeed = u64;

pub struct AribRandom {
    rr: u64,
    state: CRandom
}

impl AribRandom {
    pub fn new() -> Self {
        let mut result = Self { rr: 0, state: CRandom::new() };
        result.inirandstate();
        result
    }

    pub fn init_timestamp(timestamp: u32) -> Self {
        let mut result = Self { rr: 0, state: CRandom::new() };
        result.inirandstate_timestamp(timestamp);
        result
    }

    fn set_nth_word(&mut self, n: u8, word: u16) {
        self.rr &= !(0xFFFFu64 << (n * 16));
        self.rr |= (word as u64) << (n * 16);
    }

    fn inirandstate(&mut self) {
        self.inirandstate_timestamp(time() as u32)
    }

    fn print_state(&self) {
        println!("rr = {} {} {} {}", (self.rr >> (3 * 16)) & 0xFFFF, (self.rr >> (2 * 16)) & 0xFFFF, (self.rr >> (1 * 16)) & 0xFFFF, (self.rr >> (0 * 16)) & 0xFFFF);
    }

    fn inirandstate_timestamp(&mut self, timestamp: u32)
    {
        self.rr = 0;
        self.set_nth_word(1, sysrand_timestamp(timestamp) as u16);
        self.nextrand1();
        self.set_nth_word(0, sysrand_timestamp(timestamp) as u16);
        self.nextrand1();
        self.set_nth_word(3, 1);
    }

    fn nextrand1(&mut self) {
        let inc = 57777u64;
        let scale = 56857u64;
        let mask = 0xFFFF_FFFF_FFFF_FFFFu64;
        let mut temp = self.rr & mask;
        temp = (temp.wrapping_add(inc)) & mask;
        temp = (temp.wrapping_mul(scale)) & mask;
        self.rr = temp;
        self.set_nth_word(3, 1);
    }

    fn nextrand2(&mut self) {
        let inc = 57777u64;
        let scale = 56857u64;
        let mask = 0xFFFF_FFFF_FFFFu64;
        let mut temp = self.rr & mask;
        temp = (temp.wrapping_add(inc)) & mask;
        temp = (temp.wrapping_mul(scale)) & mask;
        self.rr = temp;
        self.set_nth_word(3, 1);
    }

    pub fn random(&mut self, m: BigInt) -> BigInt {
        let mut result = BigInt::zero();
        let len = m.to_bytes_be().1.len();
        let len16 = len / 2;
        if len <= 2 {
            self.nextrand2();
            if m.is_zero() {
                return m;
            }
            return BigInt::from(((self.rr >> 16) & 0xFFFF) % m);
        }
        for i in (0..len16).step_by(2) {
            self.nextrand1();
            let dword = ((self.rr >> 16) & 0xFFFF_FFFF) as u32;
            result |= BigInt::from(dword) << (i * 16);
        }
        result &= !(BigInt::from(0xFFFF) << (len * 8));  // somehow the leftmost 16 bit need to be cut away, idk
        result %= m;
        return result;
    }

    pub fn get_current_seed(&self) -> u64 {
        self.rr
    }

    pub fn random_seed(&mut self, seed: u64) -> u64 {
        self.rr = seed;
        self.set_nth_word(3, 1);
        self.rr
    }
}
