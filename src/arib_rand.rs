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
        self.print_state();
        println!("sysrand_timestamp: {}", sysrand_timestamp(timestamp));
        self.set_nth_word(1, sysrand_timestamp(timestamp) as u16);
        self.print_state();
        self.nextrand1();
        self.print_state();
        self.set_nth_word(0, sysrand_timestamp(timestamp) as u16);
        self.print_state();
        self.nextrand1();
        self.print_state();
        self.set_nth_word(3, 1);
        self.print_state();
    }

    fn nextrand1(&mut self) {
        let inc = 57777u64;
        let scale = 56857u64;
        let mask = 0xFFFF_FFFF_FFFFu64;
        let mut temp = self.rr & mask;
        temp = (temp + inc) & mask;
        temp = (temp + scale) & mask;
        self.rr = temp;
        self.set_nth_word(3, 1);
    }

    fn nextrand2(&mut self) {
        let inc = 57777u64;
        let scale = 56857u64;
        let mask = 0xFFFF_FFFFu64;
        let mut temp = self.rr & mask;
        temp = (temp + inc) & mask;
        temp = (temp + scale) & mask;
        self.rr = temp;
        self.set_nth_word(3, 1);
    }

    pub fn random(&mut self, m: BigInt) -> BigInt {
        let mut result = BigInt::zero();
        self.nextrand1();
        let len = m.to_bytes_be().1.len();
        for i in (0..len).step_by(2) {
            let rand_word = ((self.rr >> 8) & 0xFFFF) as u16;
            result |= BigInt::from(rand_word) << (i * 8);
        }
        result %= m;
        return result;
    }

    pub fn current_seed(&self) -> u64 {
        self.rr
    }
}
