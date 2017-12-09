/*
The spreadsheet consists of rows of apparently-random numbers. To make sure the
recovery process is on the right track, they need you to calculate the
spreadsheet's checksum. For each row, determine the difference between the
largest value and the smallest value; the checksum is the sum of all of these
differences.

For example, given the following spreadsheet:

5 1 9 5
7 5 3
2 4 6 8

The first row: 9 - 1 == 8.
The second row: 7 - 3 == 4.
The third row: 8 - 2 == 6.
The spreadsheet's checksum would be 8 + 4 + 6 = 18.
*/

#[allow(dead_code)]
struct Spreadsheet {
    rows: Vec<Vec<u32>>,
}

#[allow(dead_code)]
fn checksum(sheet: &Spreadsheet) -> u32 {
    sheet
        .rows
        .iter()
        .map(|r| r.iter().max().unwrap() - r.iter().min().unwrap())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        let input = Spreadsheet { rows: vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]] };
        assert_eq!(checksum(&input), 8 + 4 + 6);
    }

    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    #[test]
    fn main() {
        let input = Spreadsheet {
            rows: {
                let mut fin = File::open(Path::new("data/day2/input.txt")).expect("open file");
                let mut buf = String::new();
                fin.read_to_string(&mut buf).expect("read file");
                buf.lines()
                    .map(|ln| {
                        ln.split_whitespace()
                            .map(|word| word.parse().expect("parse int"))
                            .collect()
                    })
                    .collect()
            },
        };
        assert_eq!(checksum(&input), 44216);
    }
}
