
const RAND_MAX: u32 = 2147483647;

pub struct Random {
    next: u64
}

impl Random {
    pub fn new() -> Self {
        Self { next: 1 }
    }
    pub fn rand(&mut self) -> u32 {
        self.next = self.next.wrapping_mul(6364136223846793005u64).wrapping_add(1);
        return ((self.next >> 18) & 0x7FFFFFFF) as u32;
        // self.next = self.next.wrapping_mul(1103515245).wrapping_add(12345);
        // return (self.next / 65536) % RAND_MAX;
    }

    pub fn srand(&mut self, seed: u64) {
        self.next = seed;
    }
}