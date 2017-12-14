/*
Each square on the grid is allocated in a spiral pattern starting at a location
marked 1 and then counting up while spiraling outward. For example, the first
few squares are allocated like this:

17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23 -->  ..

While this is very space-efficient (no squares are skipped), requested data
must be carried back to square 1 (the location of the only access port for this
memory system) by programs that can only move up, down, left, or right. They
always take the shortest path: the Manhattan Distance between the location of
the data and square 1.
*/

#[derive(Debug, Clone, Eq, PartialEq)]
struct Location(i32, i32);

impl Location {
    fn l1(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

struct Spiral {
    location: Location,
    dir: Direction,
    len: u32,
    count: u32,
}

impl Spiral {
    pub fn new() -> Spiral {
        Spiral {
            location: Location(0, 0),
            dir: Direction::Right,
            len: 0,
            count: 1,
        }
    }

    fn shift(&mut self) {
        assert!(self.count > 0);
        match self.dir {
            Direction::Right => self.location.0 += 1,
            Direction::Up => self.location.1 += 1,
            Direction::Left => self.location.0 -= 1,
            Direction::Down => self.location.1 -= 1,
        }
        self.count -= 1;
        if self.count == 0 {
            match self.dir {
                Direction::Right => {
                    self.dir = Direction::Up;
                    self.len += 2;
                    self.count = self.len - 1;
                }
                Direction::Up => {
                    self.dir = Direction::Left;
                    self.count = self.len;
                }
                Direction::Left => {
                    self.dir = Direction::Down;
                    self.count = self.len;
                }
                Direction::Down => {
                    self.dir = Direction::Right;
                    self.count = self.len + 1;
                }
            };
        }
    }
}
impl Iterator for Spiral {
    type Item = Location;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.location.clone();
        self.shift();
        Some(pos)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn spiral_smoke() {
        assert_eq!(
            Spiral::new().take(10).collect::<Vec<_>>(),
            vec![
                Location(0, 0),
                Location(1, 0),
                Location(1, 1),
                Location(0, 1),
                Location(-1, 1),
                Location(-1, 0),
                Location(-1, -1),
                Location(0, -1),
                Location(1, -1),
                Location(2, -1),
            ]
        );
    }

    #[test]
    fn small() {
        assert_eq!(Spiral::new().nth(0).unwrap(), Location(0, 0));
        assert_eq!(Spiral::new().nth(11).unwrap(), Location(2, 1));
        assert_eq!(Spiral::new().nth(22).unwrap(), Location(0, -2));
        assert_eq!(Spiral::new().nth(1023).unwrap().l1(), 31);
    }

    #[test]
    fn main() {
        assert_eq!(Spiral::new().nth(361527 - 1).unwrap().l1(), 326);
    }
}
