
pub struct CRandom {
    front_index: usize,
    rear_index: usize,
    table_len: usize,
    state_index: usize,
    rand_type: usize,
    rand_deg: usize,
    rand_sep: usize,
    table: [i32; 32]
}

impl CRandom {
    pub fn new() -> Self {
        Self {
            front_index: 4,
            rear_index: 1,
            table_len: 32,
            state_index: 1,
            rand_type: 3,
            rand_deg: 31,
            rand_sep: 3,
            table: [
                3,
                -1726662223, 379960547, 1735697613, 1040273694, 1313901226,
                1627687941, -179304937, -2073333483, 1780058412, -1989503057,
                -615974602, 344556628, 939512070, -1249116260, 1507946756,
                -812545463, 154635395, 1388815473, -1926676823, 525320961,
                -1009028674, 968117788, -123449607, 1284210865, 435012392,
                -2017506339, -911064859, -370259173, 1132637927, 1398500161,
                -205601318
            ]
        }
    }

    pub fn print_states(&self) {
        print!("states: ");
        for i in self.table {
            print!("{} ", i);
        }
        println!();
    }

    pub fn srand(&mut self, seed: u32) {
        let mut seed = seed;
        if seed == 0 {
            seed = 1;
        }
        self.table[self.state_index] = seed as i32;
        let mut word: i32 = seed as i32;
        for i in 1..self.rand_deg {
            let hi: i64 = (word / 127773) as i64;
            let lo: i64 = (word % 127773) as i64;
            word = (16807 * lo - 2836 * hi) as i32;
            if word < 0 {
                word += 2147483647;
            }
            self.table[self.state_index + i] = word;
        }

        self.front_index = self.state_index + self.rand_sep;
        self.rear_index = self.state_index;

        for _ in 0..self.rand_deg * 10 {
            self.rand();
        }
    }

    pub fn rand(&mut self) -> u32 {
        let val: u32;
        let result: i32;

        self.table[self.front_index] = (self.table[self.front_index] as u32).wrapping_add(self.table[self.rear_index] as u32) as i32;
        val = self.table[self.front_index] as u32;
        result = (val >> 1) as i32;

        self.front_index += 1;
        if self.front_index >= self.table_len {
            self.front_index = self.state_index;
            self.rear_index += 1;
        } else {
            self.rear_index += 1;
            if self.rear_index >= self.table_len {
                self.rear_index = self.state_index;
            }
        }
        result as u32
    }
}