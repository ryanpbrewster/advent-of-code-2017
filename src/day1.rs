/*
The captcha requires you to review a sequence of digits (your puzzle input) and
find the sum of all digits that match the next digit in the list. The list is
circular, so the digit after the last digit is the first digit in the list.

For example:

1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second
digit and the third digit (2) matches the fourth digit.

1111 produces 4 because each digit (all 1) matches the next.

1234 produces 0 because no digit matches the next.

91212129 produces 9 because the only digit that matches the next one is the
last digit, 9.
*/

#[allow(dead_code)]
fn solve(xs: Vec<u32>) -> u32 {
    let mut total: u32 = 0;
    for i in 0..xs.len() {
        if xs[i] == xs[(i + 1) % xs.len()] {
            total += xs[i];
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn small() {
        assert_eq!(solve(vec![1, 1, 2, 2]), 3);
        assert_eq!(solve(vec![1, 1, 1, 1]), 4);
        assert_eq!(solve(vec![1, 2, 3, 4]), 0);
        assert_eq!(solve(vec![9, 1, 2, 1, 2, 1, 2, 9]), 9);
    }

    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    #[test]
    fn main() {
        let input = {
            let mut fin = File::open(Path::new("data/day1/input.txt")).expect("open file");
            let mut buf = String::new();
            fin.read_to_string(&mut buf).expect("read file");
            buf.chars().flat_map(|ch| ch.to_digit(10)).collect()
        };
        assert_eq!(solve(input), 1141);
    }
}
