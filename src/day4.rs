/*
A new system policy has been put in place that requires all accounts to use a
passphrase instead of simply a password. A passphrase consists of a series of
words (lowercase letters) separated by spaces.

To ensure security, a valid passphrase must contain no duplicate words.
*/

use std::collections::HashSet;

#[allow(dead_code)]
fn is_valid_pw(pw: &str) -> bool {
    let words = pw.split_whitespace().collect::<Vec<_>>();
    let uniques: HashSet<_> = words.iter().collect();
    words.len() == uniques.len()
}

#[allow(dead_code)]
fn is_valid_anagram_pw(pw: &str) -> bool {
    let words = pw.split_whitespace().collect::<Vec<&str>>();
    let uniques: HashSet<_> = words
        .iter()
        .map(|&w| {
            let mut t = w.chars().collect::<Vec<_>>();
            t.sort();
            t
        })
        .collect();
    words.len() == uniques.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert!(is_valid_pw("aa bb cc dd ee"));
        assert!(!is_valid_pw("aa bb cc dd aa"));
        assert!(is_valid_pw("aa bb cc dd aaa"));
    }

    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    #[test]
    fn main() {
        let mut fin = File::open(Path::new("data/day4/input.txt")).expect("open file");
        let mut buf = String::new();
        fin.read_to_string(&mut buf).expect("read file");
        let input = buf.lines().collect::<Vec<_>>();

        assert_eq!(input.iter().filter(|&pw| is_valid_pw(pw)).count(), 386);
        assert_eq!(
            input.iter().filter(|&pw| is_valid_anagram_pw(pw)).count(),
            208
        );
    }
}
