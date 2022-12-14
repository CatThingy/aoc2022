use std::{
    collections::HashMap,
    ops::{Add, AddAssign, Sub, SubAssign},
};

const SAND_MOVEMENTS: [Coord; 3] = [Coord::new(0, 1), Coord::new(-1, 1), Coord::new(1, 1)];
fn main() {
    let input = std::io::stdin();
    let mut rock_paths = Vec::<Vec<Coord>>::new();

    let mut max_y = 0;

    for line in input.lines() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };

        let mut path = Vec::<Coord>::new();
        for coord in line.split(" -> ") {
            let coord = coord.split_once(',').unwrap();
            let coord = Coord::new(coord.0.parse().unwrap(), coord.1.parse().unwrap());

            max_y = max_y.max(coord.y);

            path.push(coord);
        }

        rock_paths.push(path);
    }
    let height = max_y + 2;

    dbg!(height);

    let mut map = HashMap::<Coord, Tile>::new();

    for path in rock_paths {
        let mut iter = path.windows(2);
        while let Some(&[a, b]) = iter.next() {
            if a.x == b.x {
                let larger = a.y.max(b.y);
                let smaller = a.y.min(b.y);
                for y in smaller..=larger {
                    map.insert(Coord::new(a.x, y), Tile::Stone);
                }
            } else if a.y == b.y {
                let larger = a.x.max(b.x);
                let smaller = a.x.min(b.x);
                for x in smaller..=larger {
                    map.insert(Coord::new(x, a.y), Tile::Stone);
                }
            } else {
                unreachable!()
            }
        }
    }

    'a: loop {
        let mut sand_coord = Coord::new(500, 0);

        'b: loop {
            let mut stopped = true;
            for direction in SAND_MOVEMENTS {
                let next_pos = sand_coord + direction;
                if next_pos.y >= height {
                    break;
                }
                if let Some(tile) = map.get(&next_pos) {
                    if tile == &Tile::Empty {
                        sand_coord += direction;
                        stopped = false;
                        break;
                    }
                } else {
                    sand_coord += direction;
                    stopped = false;
                    break;
                }
            }
            if stopped {
                break 'b;
            }
        }
        map.insert(sand_coord, Tile::Sand);

        if sand_coord == Coord::new(500, 0) {
            break 'a;
        }
    }

    let mut min_x = 20000;
    let mut max_x = 0;

    for (coord, _) in map.iter() {
        min_x = min_x.min(coord.x);
        max_x = max_x.max(coord.x);
    }

    draw_map(&map, min_x, max_x, height);

    let count = map.into_iter().filter(|(_, v)| v == &Tile::Sand).count();
    println!("{count}");
}

fn draw_map(map: &HashMap<Coord, Tile>, min_x: i32, max_x: i32, height: i32) {
    for y in 0..height {
        for x in min_x..=max_x {
            let output = match map.get(&Coord::new(x, y)) {
                Some(Tile::Empty) | None => '.',
                Some(Tile::Stone) => '█',
                Some(Tile::Sand) => 'o',
            };

            print!("{output}");
        }
        print!("\n");
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Stone,
    Sand,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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
