/*
The message includes a list of the offsets for each jump. Jumps are relative:
-1 moves to the previous instruction, and 2 skips the next one. Start at the
first instruction in the list. The goal is to follow the jumps until one leads
outside the list.

In addition, these instructions are a little strange; after each jump, the
offset of that instruction increases by 1. So, if you come across an offset of
3, you would move three instructions forward, but change it to a 4 for the next
time it is encountered.
*/

#[allow(dead_code)]
fn solve(mut jumps: Vec<i32>) -> usize {
    let mut cur: i32 = 0;
    let mut count = 0;
    while 0 <= cur && cur < jumps.len() as i32 {
        let idx = cur as usize;
        cur += jumps[idx];
        jumps[idx] += 1;
        count += 1;
    }
    count
}

#[allow(dead_code)]
fn solve2(mut jumps: Vec<i32>) -> usize {
    let mut cur: i32 = 0;
    let mut count = 0;
    while 0 <= cur && cur < jumps.len() as i32 {
        let idx = cur as usize;
        cur += jumps[idx];
        jumps[idx] += if jumps[idx] < 3 { 1 } else { -1 };
        count += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert_eq!(solve(vec![0, 3, 0, 1, -3]), 5);
        assert_eq!(solve2(vec![0, 3, 0, 1, -3]), 10);
    }

    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    #[test]
    fn main() {
        let input: Vec<i32> = {
            let mut fin = File::open(Path::new("data/day5/input.txt")).expect("open file");
            let mut buf = String::new();
            fin.read_to_string(&mut buf).expect("read file");
            buf.lines()
                .map(|ln| ln.parse::<i32>().expect("parse int"))
                .collect()
        };
        assert_eq!(solve(input.clone()), 374269);
        assert_eq!(solve2(input), 27720699);
    }
}
