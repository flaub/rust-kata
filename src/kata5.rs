use crate::bitvec::BitVec;
use std::default::Default;
use std::f32::consts::LN_2;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker;

pub struct BloomFilter<H> {
    bitmap: BitVec,
    nhashes: usize,
    hasher: marker::PhantomData<H>,
}

impl<H: Hasher + Default> BloomFilter<H> {
    pub fn new(capacity: usize, error_rate: f32) -> BloomFilter<H> {
        let cap = capacity as f32;
        let nbits = (cap * error_rate.ln().abs()).ceil() / (LN_2 * LN_2);
        let nhashes = (nbits * LN_2 / cap).ceil() as usize;
        // println!("Capacity: {} Bits: {} Hashes: {}", cap, nbits, nhashes);
        BloomFilter::<H> {
            bitmap: BitVec::from_elem(nbits as usize, false),
            nhashes: nhashes,
            hasher: marker::PhantomData,
        }
    }

    pub fn bitmap(&self) -> &BitVec {
        &self.bitmap
    }

    pub fn clear(&mut self) {
        self.bitmap.clear()
    }

    pub fn get<T: Hash>(&self, key: &T) -> bool {
        let digest = self.compute_digest(key);
        for i in 0..self.nhashes {
            let ix = self.compute_index(i, digest);
            if !self.bitmap.get(ix).unwrap() {
                return false;
            }
        }
        return true;
    }

    pub fn set<T: Hash>(&mut self, key: &T) {
        let digest = self.compute_digest(key);
        for i in 0..self.nhashes {
            let ix = self.compute_index(i, digest);
            self.bitmap.set(ix, true);
        }
    }

    fn compute_digest<T: Hash>(&self, key: &T) -> u64 {
        let mut h: H = Default::default();
        key.hash(&mut h);
        return h.finish();
    }

    // see: https://willwhim.wpengine.com/2011/09/03/producing-n-hash-functions-by-hashing-only-once/
    fn compute_index(&self, i: usize, digest: u64) -> usize {
        let a = digest & u32::max_value() as u64;
        let b = digest >> 32;
        let x = a + b * (i as u64);
        // println!("i: {} digest: {:#018X} a: {:#010X} b: {:#010X} x: {:#012X}", i, digest, a, b, x);
        return (x % self.bitmap.len() as u64) as usize;
    }
}

#[cfg(test)]
mod test {
    use super::BloomFilter;
    use std::fs::File;
    use std::collections::hash_map::DefaultHasher;
    use std::io::Seek;
    use std::io::SeekFrom;
    use std::io::{BufRead, BufReader};

    struct Stats {
        true_pos: f32,
        true_neg: f32,
        false_pos: f32,
        false_neg: f32,
    }

    impl Stats {
        pub fn new() -> Stats {
            Stats {
                true_pos: 0_f32,
                true_neg: 0_f32,
                false_pos: 0_f32,
                false_neg: 0_f32,
            }
        }

        pub fn score(&mut self, actual: bool, expected: bool) {
            if expected {
                if actual {
                    self.true_pos += 1_f32;
                } else {
                    self.false_neg += 1_f32;
                }
            } else {
                if actual {
                    self.false_pos += 1_f32;
                } else {
                    self.true_neg += 1_f32;
                }
            }
        }

        pub fn report(&self) -> f32 {
            let rate = self.false_pos / (self.false_pos + self.true_neg);

            println!("True positives:      {:>7}", self.true_pos);
            println!("True negatives:      {:>7}", self.true_neg);
            println!("False positives:     {:>7}", self.false_pos);
            println!("False negatives:     {:>7}", self.false_neg);
            println!("False positive rate: {:>7}", rate);

            assert!(self.false_neg == 0_f32);
            return rate;
        }
    }

    #[test]
    fn test_bloom_filter() {
        let capacity = 50000;
        let error_rate = 0.05;
        let mut bf = BloomFilter::<DefaultHasher>::new(capacity, error_rate);

        let mut fin = File::open("/usr/share/dict/words").unwrap();
        {
            let reader = BufReader::new(&fin);
            let lines = reader.lines();

            for (i, item) in lines.enumerate() {
                if i < capacity * 2 && i % 2 == 0 {
                    let word = item.unwrap();
                    // println!("{}", word);
                    bf.set(&word);
                }
            }
        }

        let _ = fin.seek(SeekFrom::Start(0));
        let mut stats = Stats::new();

        {
            let reader = BufReader::new(&fin);
            let lines = reader.lines();

            for (i, item) in lines.enumerate() {
                if i < capacity * 2 {
                    let word = item.unwrap();
                    // println!("{}", word);
                    stats.score(bf.get(&word), i % 2 == 0);
                }
            }
        }

        stats.report();
        // assert!(rate < error_rate);
    }
}
