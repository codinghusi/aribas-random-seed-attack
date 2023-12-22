use std::ops::Range;
use num_traits::Num;

pub type Ranges = Vec<Range<u32>>;

pub struct OneChunk {
    ranges: Ranges,
    current_index: usize,
    current_val: u32,
    pub first: u32,
    finished: bool,
}

impl OneChunk {
    pub fn new(ranges: Ranges) -> Self {
        if ranges.len() == 0 {
            panic!("len = 0 not supported");
        }
        Self {
            current_val: ranges[0].start,
            first: ranges[0].start,
            ranges,
            current_index: 0,
            finished: false,
        }
    }
}

impl Iterator for OneChunk {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let ret = self.current_val;
        self.current_val += 1;
        if self.current_val > self.ranges[self.current_index].end {
            self.current_index += 1;
            if self.current_index >= self.ranges.len() {
                self.finished = true;
            } else {
                self.current_val = self.ranges[self.current_index].start;
            }
        }
        Some(ret)
    }
}

pub struct ChunkedRanges {
    chunk_size: u32,
    ranges: Ranges,
    current_index: usize,
    current_val: u32,
    pub first: u32
}

impl ChunkedRanges {
    pub fn new(ranges: Ranges, chunk_size: u32) -> Self {
        if ranges.len() == 0 {
            panic!("didn't want to implement a case where you have zero elements in your ranges list");
        }
        Self {
            first: ranges[0].start,
            current_val: ranges[0].start,
            ranges,
            current_index: 0,
            chunk_size,
        }
    }
}

impl Iterator for ChunkedRanges {
    type Item = Ranges;

    fn next(&mut self) -> Option<Self::Item> {
        let mut range = &self.ranges[self.current_index];
        let start = self.current_val;
        let mut result = vec![];
        let mut rest = self.chunk_size;
        let mut i = start;
        while i + rest >= range.end {
            if i != range.end {
                result.push(i..range.end);
            }
            i += range.end - i;
            rest -= range.end - i;
            self.current_index += 1;
            if self.current_index >= self.ranges.len() {
                if result.len() > 0 {
                    return Some(result);
                }
                return None;
            }
            range = &self.ranges[self.current_index];
        }
        self.current_val = i+rest;
        result.push(i..i+rest);
        Some(result)
    }
}

// pub fn chunk_ranges(ranges: &Ranges) {
//
// }

pub fn rearrange_ranges(mut ranges: Ranges) -> Ranges {
    if ranges.len() == 0 {
        return ranges;
    }
    ranges.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
    let mut result = vec![];
    let mut current: Option<Range<u32>> = None;
    for range in ranges {
        current = if let Some(current) = current {
            if (current.contains(&range.start) && current.end < range.end) || current.end + 1 == range.start {
                Some(current.start..range.end)
            } else {
                result.push(current);
                Some(range)
            }
        } else {
            Some(range)
        }
    }
    if let Some(current) = current {
        result.push(current);
    }
    result
}