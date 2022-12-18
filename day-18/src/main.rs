use std::{collections::HashSet, ops::Add};

const DIRECTIONS: [Coord; 6] = [
    Coord::new(0, 0, 1),
    Coord::new(0, 0, -1),
    Coord::new(0, 1, 0),
    Coord::new(0, -1, 0),
    Coord::new(1, 0, 0),
    Coord::new(-1, 0, 0),
];

fn main() {
    let input = std::io::stdin();

    let mut cubes = HashSet::<Coord>::new();

    for line in input.lines() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };

        let coords = line
            .split(',')
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        cubes.insert(Coord::new(coords[0], coords[1], coords[2]));
    }

    let mut counter = 0;

    for cube in &cubes {
        for direction in DIRECTIONS {
            if !cubes.contains(&(*cube + direction)) {
                counter += 1;
            }
        }
    }

    println!("{counter}");
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
