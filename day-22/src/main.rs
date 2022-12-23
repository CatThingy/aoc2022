use std::{
    collections::{HashMap, VecDeque},
    ops::{Add, Neg, Sub},
};

const CUBE_DIM: u32 = 50;

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

    let mut cube_face_pos = Vec::<Coord>::new();

    for (y, line) in input.lines().enumerate() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };

        let mut area_start = u32::MAX;
        let mut area_end = u32::MIN;

        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    board.insert(Coord::new(x as i32, y as i32), Tile::Blocked);
                    area_start = (x as u32).min(area_start);
                }
                '.' => {
                    board.insert(Coord::new(x as i32, y as i32), Tile::Empty);

                    if y == 0 && start_x == None {
                        start_x = Some(x);
                    }
                    area_start = (x as u32).min(area_start);
                }
                _ => {}
            }
            area_end = area_end.max(x as u32);
        }

        for i in area_start..area_end {
            let face = Coord::new(i as i32, y as i32).to_cubemap_pos();

            if !cube_face_pos.contains(&face) {
                cube_face_pos.push(face);
            }
        }
    }

    let cube_mapping = HashMap::<Coord, Face>::from([
        (Coord::new(1, 0), Face::Top),
        (Coord::new(2, 0), Face::Right),
        (Coord::new(1, 1), Face::Front),
        (Coord::new(1, 2), Face::Bottom),
        (Coord::new(0, 2), Face::Left),
        (Coord::new(0, 3), Face::Back),
    ]);

    let edges = HashMap::from([
        (
            Face::Top,
            [
                (Face::Right, 0),
                (Face::Front, 0),
                (Face::Left, 2),
                (Face::Back, 1),
            ],
        ),
        (
            Face::Right,
            [
                (Face::Bottom, 2),
                (Face::Front, 1),
                (Face::Top, 0),
                (Face::Back, 0),
            ],
        ),
        (
            Face::Front,
            [
                (Face::Right, 3),
                (Face::Bottom, 0),
                (Face::Left, 3),
                (Face::Top, 0),
            ],
        ),
        (
            Face::Bottom,
            [
                (Face::Right, 2),
                (Face::Back, 1),
                (Face::Left, 0),
                (Face::Front, 0),
            ],
        ),
        (
            Face::Left,
            [
                (Face::Bottom, 0),
                (Face::Back, 0),
                (Face::Top, 2),
                (Face::Front, 1),
            ],
        ),
        (
            Face::Back,
            [
                (Face::Bottom, 3),
                (Face::Right, 0),
                (Face::Top, 3),
                (Face::Left, 0),
            ],
        ),
    ]);

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

    dbg!(pos.tile);
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
                    let relative_pos = pos.tile - pos.tile.to_cubemap_pos().from_cubemap_pos();

                    let face = &cube_mapping[&pos.tile.to_cubemap_pos()];

                    let (next_face, turn) = &edges[face][pos.facing];

                    let new_relative_pos = match turn {
                        0 => match pos.facing {
                            0 => Coord {
                                x: 0,
                                y: relative_pos.y,
                            },
                            1 => Coord {
                                x: relative_pos.x,
                                y: 0,
                            },
                            2 => Coord {
                                x: CUBE_DIM as i32 - 1,
                                y: relative_pos.y,
                            },
                            3 => Coord {
                                x: relative_pos.x,
                                y: CUBE_DIM as i32 - 1,
                            },
                            _ => unreachable!(),
                        },
                        1 => match pos.facing {
                            0 => Coord {
                                x: relative_pos.y,
                                y: CUBE_DIM as i32 - 1,
                            },
                            1 => Coord {
                                x: CUBE_DIM as i32 - 1,
                                y: relative_pos.x,
                            },
                            2 => Coord {
                                x: relative_pos.y,
                                y: 0,
                            },
                            3 => Coord {
                                x: 0,
                                y: relative_pos.x,
                            },
                            _ => unreachable!(),
                        },
                        2 => match pos.facing {
                            0 => Coord {
                                x: CUBE_DIM as i32 - 1,
                                y: CUBE_DIM as i32 - 1 - relative_pos.y,
                            },
                            1 => Coord {
                                x: CUBE_DIM as i32 - 1 - relative_pos.x,
                                y: CUBE_DIM as i32 - 1,
                            },
                            2 => Coord {
                                x: 0,
                                y: CUBE_DIM as i32 - 1 - relative_pos.y,
                            },
                            3 => Coord {
                                x: CUBE_DIM as i32 - 1 - relative_pos.x,
                                y: 0,
                            },
                            _ => unreachable!(),
                        },
                        3 => match pos.facing {
                            0 => Coord {
                                x: relative_pos.y,
                                y: CUBE_DIM as i32 - 1,
                            },
                            1 => Coord {
                                x: CUBE_DIM as i32 - 1,
                                y: relative_pos.x,
                            },
                            2 => Coord {
                                x: relative_pos.y,
                                y: 0,
                            },
                            3 => Coord {
                                x: 0,
                                y: relative_pos.x,
                            },
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    let (new_coord, _) =
                        cube_mapping.iter().find(|(_, v)| *v == next_face).unwrap();
                    let next_search = new_coord.from_cubemap_pos() + new_relative_pos;
                    // dbg!((turn, pos.facing, next_face, face));
                    // dbg!((relative_pos, new_relative_pos));
                    match board.get(&next_search).unwrap() {
                        Tile::Blocked => {
                            break;
                        }
                        Tile::Empty => {
                            dbg!(pos.tile);
                            pos.tile = next_search;
                            dbg!(pos.tile);
                            pos.facing = (((pos.facing as i32 + turn) + 4) % 4) as usize;
                        }
                    }
                }
            }
        }
        dbg!(pos.tile);
        pos.facing = (((pos.facing as i32 + turn) + 4) % 4) as usize;
    }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Face {
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
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

    const fn to_cubemap_pos(self) -> Coord {
        Coord {
            x: self.x / CUBE_DIM as i32,
            y: self.y / CUBE_DIM as i32,
        }
    }

    const fn from_cubemap_pos(self) -> Coord {
        Coord {
            x: self.x * CUBE_DIM as i32,
            y: self.y * CUBE_DIM as i32,
        }
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
