use std::ops::{Add, AddAssign, Sub, SubAssign};

const SAND_MOVEMENTS: [Coord; 3] = [Coord::new(0, 1), Coord::new(-1, 1), Coord::new(1, 1)];
fn main() {
    let input = std::io::stdin();
    let mut rock_paths = Vec::<Vec<Coord>>::new();

    let mut min_x = 20000;
    let mut max_x = 0;
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

            min_x = min_x.min(coord.x);
            max_x = max_x.max(coord.x);
            max_y = max_y.max(coord.y);

            path.push(coord);
        }

        rock_paths.push(path);
    }
    let width = max_x - min_x + 1;
    let height = max_y + 1;

    dbg!((min_x, max_x));
    dbg!((width, height));
    let mut map = vec![Tile::Empty; (width * height) as usize];

    for path in rock_paths {
        let mut iter = path.windows(2);
        while let Some(&[a, b]) = iter.next() {
            if a.x == b.x {
                let larger = a.y.max(b.y);
                let smaller = a.y.min(b.y);
                for y in smaller..=larger {
                    map[(y * width + a.x - min_x) as usize] = Tile::Stone;
                }
            } else if a.y == b.y {
                let larger = a.x.max(b.x);
                let smaller = a.x.min(b.x);
                for x in smaller..=larger {
                    map[(a.y * width + x - min_x) as usize] = Tile::Stone;
                }
            } else {
                unreachable!()
            }
        }
    }

    // draw_map(&map, width as usize, height as usize);

    'a: loop {
        let mut sand_coord = Coord::new(500, 0);

        'b: loop {
            let mut stopped = true;
            for direction in SAND_MOVEMENTS {
                let next_pos = sand_coord + direction;
                if let Some(tile) = map.get((next_pos.y * width + next_pos.x - min_x) as usize) {
                    if tile == &Tile::Empty {
                        sand_coord += direction;
                        stopped = false;
                        break;
                    }
                } else {
                    break 'a;
                }
            }
            if stopped {
                break 'b;
            }
        }
        map[(sand_coord.y * width + sand_coord.x - min_x) as usize] = Tile::Sand;
    }

    draw_map(&map, width as usize, height as usize);

    let count = map.into_iter().filter(|v| v == &Tile::Sand).count();
    println!("{count}");
}

fn draw_map(map: &[Tile], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let output = match map[y * width + x] {
                Tile::Empty => '.',
                Tile::Stone => '█',
                Tile::Sand => 'o',
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
