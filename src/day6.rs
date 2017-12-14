/*
The reallocation routine operates in cycles. In each cycle, it finds the memory
bank with the most blocks (ties won by the lowest-numbered memory bank) and
redistributes those blocks among the banks. To do this, it removes all of the
blocks from the selected bank, then moves to the next (by index) memory bank
and inserts one of the blocks. It continues doing this until it runs out of
blocks; if it reaches the last memory bank, it wraps around to the first one.

The debugger would like to know how many redistributions can be done before a
blocks-in-banks configuration is produced that has been seen before.
*/

use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Configuration {
    banks: Vec<usize>,
}

impl Configuration {
    pub fn redistribute(&mut self) {
        let idx = self.source_idx();
        let len = self.banks.len();
        let count = self.banks[idx];
        self.banks[idx] = 0;
        for i in 0..count {
            self.banks[(idx + i + 1) % len] += 1;
        }
    }

    fn source_idx(&self) -> usize {
        let mut idx = 0;
        let mut val = self.banks[0];
        for i in 1..self.banks.len() {
            if self.banks[i] > val {
                idx = i;
                val = self.banks[i];
            }
        }
        idx
    }
}

struct ChainLength {
    repeating: usize,
    total: usize,
}

fn chain_length(seed: Configuration) -> ChainLength {
    let mut seen = HashMap::new();

    let mut conf = seed;
    let mut count = 0;
    while !seen.contains_key(&conf) {
        count += 1;
        seen.insert(conf.clone(), count);
        conf.redistribute();
    }

    ChainLength {
        total: count,
        repeating: count - seen[&conf] + 1,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        let seed = Configuration { banks: vec![0, 2, 7, 0] };
        let len = chain_length(seed);
        assert_eq!(len.total, 5);
        assert_eq!(len.repeating, 4);
    }

    #[test]
    fn main() {
        let seed = Configuration { banks: vec![2, 8, 8, 5, 4, 2, 3, 1, 5, 5, 1, 2, 15, 13, 5, 14] };
        let len = chain_length(seed);
        assert_eq!(len.total, 3156);
        assert_eq!(len.repeating, 1610);
    }
}
