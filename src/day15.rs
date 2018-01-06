struct Generator {
    value: u64,
    multiplier: u64,
    modulus: u64,
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value * self.multiplier) % self.modulus;
        Some(self.value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        let a = Generator {
            value: 65,
            multiplier: 16807,
            modulus: 2147483647,
        };
        assert_eq!(
            a.take(5).collect::<Vec<_>>(),
            vec![1092455, 1181022009, 245556042, 1744312007, 1352636452]
        );

        let b = Generator {
            value: 8921,
            multiplier: 48271,
            modulus: 2147483647,
        };
        assert_eq!(
            b.take(5).collect::<Vec<_>>(),
            vec![430625591, 1233683848, 1431495498, 137874439, 285222916]
        );
    }

    #[test]
    fn small() {
        let a = Generator {
            value: 65,
            multiplier: 16807,
            modulus: 2147483647,
        };
        let b = Generator {
            value: 8921,
            multiplier: 48271,
            modulus: 2147483647,
        };
        assert_eq!(
            a.zip(b)
                .take(40_000_000)
                .filter(|&(m, n)| (m & 0xFFFF) == (n & 0xFFFF))
                .count(),
            588
        );
    }

    #[test]
    fn part1() {
        let a = Generator {
            value: 277,
            multiplier: 16807,
            modulus: 2147483647,
        };
        let b = Generator {
            value: 349,
            multiplier: 48271,
            modulus: 2147483647,
        };
        assert_eq!(
            a.zip(b)
                .take(40_000_000)
                .filter(|&(m, n)| (m & 0xFFFF) == (n & 0xFFFF))
                .count(),
            592
        );
    }

    #[test]
    fn part2() {
        let a = Generator {
            value: 277,
            multiplier: 16807,
            modulus: 2147483647,
        }.filter(|&m| m % 4 == 0);
        let b = Generator {
            value: 349,
            multiplier: 48271,
            modulus: 2147483647,
        }.filter(|&n| n % 8 == 0);
        assert_eq!(
            a.zip(b)
                .take(5_000_000)
                .filter(|&(m, n)| (m & 0xFFFF) == (n & 0xFFFF))
                .count(),
            320
        );
    }
}
