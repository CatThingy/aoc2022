use std::{
    collections::HashSet,
    ops::{Add, AddAssign, SubAssign, Sub},
};

fn main() {
    let input = std::io::stdin();

    let mut head = Coord::new(0, 0);
    let mut tail = Coord::new(0, 0);

    let mut visited = HashSet::<Coord>::new();

    for line in input.lines() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };

        let command = line.split_whitespace().collect::<Vec<_>>();

        let direction = match command[0] {
            "U" => Coord::new(0, -1),
            "D" => Coord::new(0, 1),
            "L" => Coord::new(-1, 0),
            "R" => Coord::new(1, 0),

            _ => unreachable!(),
        };

        let amount = command[1].parse::<i32>().unwrap();

        for _ in 0..amount {
            head += direction;

            if head.distance(tail) > 1 {
                tail = head - direction;
            }
            visited.insert(tail);

            dbg!((head, tail));
        }
    }

    dbg!(visited.len());
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn distance(&self, other: Coord) -> i32 {
        (self.x - other.x).abs().max((self.y - other.y).abs())
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
