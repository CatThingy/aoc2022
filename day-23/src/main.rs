use std::{
    collections::{HashSet, VecDeque},
    ops::{Add, Neg, Sub},
};

const DIRECTIONS: [Coord; 8] = [
    Coord::new(0, -1),
    Coord::new(1, -1),
    Coord::new(1, 1),
    Coord::new(0, 1),
    Coord::new(-1, 1),
    Coord::new(1, 0),
    Coord::new(-1, 0),
    Coord::new(-1, -1),
];

const NORTH_CHECK: [Coord; 3] = [Coord::new(-1, -1), Coord::new(0, -1), Coord::new(1, -1)];
const SOUTH_CHECK: [Coord; 3] = [Coord::new(1, 1), Coord::new(0, 1), Coord::new(-1, 1)];
const EAST_CHECK: [Coord; 3] = [Coord::new(1, -1), Coord::new(1, 0), Coord::new(1, 1)];
const WEST_CHECK: [Coord; 3] = [Coord::new(-1, 1), Coord::new(-1, 0), Coord::new(-1, -1)];

fn main() {
    let input = std::io::stdin();

    let mut map = HashSet::<Coord>::new();
    for (y, line) in input.lines().enumerate() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };

        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    map.insert(Coord {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                _ => {}
            }
        }
    }

    let mut checks = VecDeque::from([
        (NORTH_CHECK, Coord::new(0, -1)),
        (SOUTH_CHECK, Coord::new(0, 1)),
        (WEST_CHECK, Coord::new(-1, 0)),
        (EAST_CHECK, Coord::new(1, 0)),
    ]);

    for i in 0.. {
        let mut proposed_movements = Vec::<(Coord, Coord)>::new();
        for coord in map.iter() {
            let mut valid = true;
            for dir in DIRECTIONS {
                if map.contains(&(*coord + dir)) {
                    valid = false;
                }
            }
            if valid {
                continue;
            }

            for (check, dir) in &checks {
                valid = true;

                for c in check {
                    if map.contains(&(*coord + *c)) {
                        valid = false;
                    }
                }

                if valid {
                    proposed_movements.push((coord.clone(), coord.clone() + *dir));
                    break;
                }
            }
        }

        if proposed_movements.is_empty() {
            println!("{}", i + 1);
            break;
        }

        let front = checks.pop_front().unwrap();
        checks.push_back(front);

        let mut destinations = HashSet::<Coord>::new();
        let mut cancelled = HashSet::<Coord>::new();

        for (_, to) in &proposed_movements {
            if !destinations.insert(to.clone()) {
                cancelled.insert(to.clone());
            }
        }
        for (from, to) in proposed_movements {
            if !cancelled.contains(&to) {
                map.remove(&from);
                map.insert(to);
            }
        }
    }
}

fn print_map(map: &HashSet<Coord>, height: i32, width: i32) {
    for y in -height..=height {
        for x in -width..=width {
            let output = match map.contains(&Coord::new(x, y)) {
                true => "#",
                false => ".",
            };

            print!("{output}");
        }
        print!("\n");
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Neg for Coord {
    type Output = Coord;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
