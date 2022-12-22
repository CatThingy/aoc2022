use std::{
    collections::HashMap,
    ops::{Add, Neg, Sub},
};

static DIRECTIONS: [Coord; 4] = [
    Coord::new(1, 0),
    Coord::new(0, 1),
    Coord::new(-1, 0),
    Coord::new(0, -1),
];

fn main() {
    let input = std::io::stdin();

    let mut board = HashMap::<Coord, Tile>::new();

    let mut start_x = None;

    for (y, line) in input.lines().enumerate() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };

        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    board.insert(Coord::new(x as i32, y as i32), Tile::Blocked);
                }
                '.' => {
                    board.insert(Coord::new(x as i32, y as i32), Tile::Empty);

                    if y == 0 {
                        start_x = Some(x);
                    }
                }
                _ => {}
            }
        }
    }

    let mut path = String::new();
    std::io::stdin().read_line(&mut path).unwrap();

    let turns = path.clone();

    let path = path
        .trim()
        .split(['L', 'R'])
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut turns = turns
        .trim()
        .split(|v: char| v.is_ascii_digit())
        .filter_map(|v| match v {
            "L" => Some(-1),
            "R" => Some(1),
            _ => None,
        })
        .collect::<Vec<_>>();

    let mut pos = Position {
        tile: Coord::new(start_x.unwrap() as i32, 0),
        facing: 0,
    };

    
    // one more move input than turn input
    turns.push(0);

    for (amnt, turn) in path.into_iter().zip(turns) {
        for _ in 0..amnt {
            let next_pos = pos.tile + DIRECTIONS[pos.facing];

            let new_tile = board.get(&next_pos);

            match new_tile {
                Some(Tile::Empty) => {
                    pos.tile = pos.tile + DIRECTIONS[pos.facing];
                }
                Some(Tile::Blocked) => {
                    break;
                }
                None => {
                    let search_direction = -DIRECTIONS[pos.facing];

                    let mut next_search = pos.tile + search_direction;

                    while board.get(&(next_search + search_direction)).is_some() {
                        next_search = next_search + search_direction;
                    }

                    match board.get(&next_search).unwrap() {
                        Tile::Blocked => {
                            break;
                        }
                        Tile::Empty => {
                            pos.tile = next_search;
                        }
                    }
                }
            }

        }
        pos.facing = (((pos.facing as i32 + turn) + 4) % 4) as usize;
        dbg!(&pos);
    }

    dbg!(pos.tile.y + 1, pos.tile.x + 1);
    println!(
        "{}",
        (pos.tile.y + 1) * 1000 + (pos.tile.x + 1) * 4 + pos.facing as i32
    );
}

#[derive(Debug)]
struct Position {
    tile: Coord,
    facing: usize,
}

#[derive(PartialEq)]
enum Tile {
    Blocked,
    Empty,
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
