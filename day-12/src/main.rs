use std::{
    collections::{HashMap, VecDeque},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Sub, SubAssign},
};

const DIRECTIONS: [Coord; 4] = [
    Coord::new(-1, 0),
    Coord::new(1, 0),
    Coord::new(0, -1),
    Coord::new(0, 1),
];

fn main() {
    let input = std::io::stdin();

    let mut heightmap = HashMap::<Coord, u8>::new();
    let mut edges = HashMap::<Coord, Vec<Coord>>::new();

    let mut start = Coord::new(-1, -1);
    let mut end = Coord::new(-1, -1);
    for (y, line) in input.lines().enumerate() {
        let Ok(line) = line else { break; };

        for (x, char) in line.chars().enumerate() {
            let coord = Coord::new(x as i32, y as i32);
            let height = match char {
                'S' => {
                    start = coord;
                    0
                }
                'E' => {
                    end = coord;
                    25
                }
                'a'..='z' => char as u8 - 'a' as u8,
                _ => unreachable!(),
            };

            heightmap.insert(coord, height);
        }
    }

    for (coord, height) in heightmap.iter() {
        let mut new_edges = Vec::with_capacity(4);
        for direction in DIRECTIONS {
            match heightmap.get(&(*coord + direction)) {
                Some(neighbour_height) if height >= neighbour_height || height.abs_diff(*neighbour_height) == 1 => {
                    new_edges.push(*coord + direction);
                }
                _ => {}
            }
        }
        edges.insert(*coord, new_edges);
    }

    let mut explored = HashMap::<Coord, SearchNode>::new();
    let mut exploration_queue = VecDeque::<SearchNode>::new();

    let begin_node = SearchNode {
        coord: start,
        prev: None,
    };
    explored.insert(start, begin_node.clone());

    exploration_queue.push_back(begin_node);

    let mut dest = SearchNode {
        coord: Coord::new(-1, -1),
        prev: None,
    };
    while let Some(next) = exploration_queue.pop_front() {
        if next.coord == end {
            dest = next;
            break;
        }

        for edge_coord in edges.get(&next.coord).unwrap() {
            if !explored.contains_key(edge_coord) {
                let new_node = SearchNode {
                    coord: *edge_coord,
                    prev: Some(next.coord),
                };
                explored.insert(*edge_coord, new_node.clone());
                exploration_queue.push_back(new_node);
            }
        }
    }

    dbg!(&dest);

    let mut path_length = 0;
    while let Some(prev) = dest.prev {
        path_length += 1;
        dest = explored.get(&prev).unwrap().clone();
    }

    dbg!(path_length);
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct SearchNode {
    coord: Coord,
    prev: Option<Coord>,
}
impl Hash for SearchNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coord.hash(state);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
