use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::{Add, Neg, Sub},
};

fn main() {
    let input = std::io::stdin();

    let mut map = HashMap::<Coord, Tile>::new();
    let mut blizzards = Vec::<(Coord, Blizzard)>::new();

    let mut start_point = Coord::new(i32::MAX, i32::MAX);
    let mut end_point = Coord::new(i32::MIN, i32::MIN);

    for (y, line) in input.lines().enumerate() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };
        for (x, char) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            match char {
                '#' => {
                    map.insert(Coord { x, y }, Tile::Wall);
                }
                '.' => {
                    map.insert(Coord { x, y }, Tile::Empty);

                    start_point.x = start_point.x.min(x);
                    start_point.y = start_point.y.min(y);

                    end_point.x = start_point.x.max(x);
                    end_point.y = start_point.y.max(y);
                }
                '^' => {
                    map.insert(Coord { x, y }, Tile::Empty);
                    blizzards.push((Coord { x, y }, Blizzard::Up));
                }
                'v' => {
                    map.insert(Coord { x, y }, Tile::Empty);
                    blizzards.push((Coord { x, y }, Blizzard::Down));
                }
                '<' => {
                    map.insert(Coord { x, y }, Tile::Empty);
                    blizzards.push((Coord { x, y }, Blizzard::Left));
                }
                '>' => {
                    map.insert(Coord { x, y }, Tile::Empty);
                    blizzards.push((Coord { x, y }, Blizzard::Right));
                }
                _ => unreachable!(),
            }
        }
    }

    let max_extents = Coord::new(end_point.x, end_point.y - 1);

    let mut exploration_queue = VecDeque::<SearchNode>::new();
    let mut explored = HashSet::<SearchNode>::new();
    let mut blizzard_cache = HashMap::<u32, Vec<(Coord, Blizzard)>>::new();
    let begin_node = SearchNode {
        coord: start_point,
        count: 0,
        state: SearchState::ToEnd,
    };

    exploration_queue.push_back(begin_node);
    blizzard_cache.insert(0, blizzards);

    let mut dest = SearchNode {
        coord: Coord::new(-1, -1),
        count: 0,
        state: SearchState::ToStart,
    };
    while let Some(mut next) = exploration_queue.pop_front() {
        match next.state {
            SearchState::ToEnd => {
                if next.coord == end_point {
                    next.state = SearchState::ToStart;
                    exploration_queue.clear();
                }
            }
            SearchState::ToStart => {
                if next.coord == start_point {
                    next.state = SearchState::ToFinish;
                    exploration_queue.clear();
                }
            }
            SearchState::ToFinish => {
                if next.coord == end_point {
                    dest = next;
                    break;
                }
            }
        }

        let blizzards = match blizzard_cache.get(&(next.count + 1)) {
            Some(v) => v.clone(),
            None => {
                let mut blizzards = blizzard_cache.get(&next.count).unwrap().clone();

                for (ref mut coord, blizzard) in &mut blizzards {
                    match blizzard {
                        Blizzard::Up => {
                            *coord = *coord + Coord::new(0, -1);
                        }
                        Blizzard::Down => {
                            *coord = *coord + Coord::new(0, 1);
                        }
                        Blizzard::Left => {
                            *coord = *coord + Coord::new(-1, 0);
                        }
                        Blizzard::Right => {
                            *coord = *coord + Coord::new(1, 0);
                        }
                    }
                    if coord.x > max_extents.x {
                        coord.x = 1;
                    }
                    if coord.x < 1 {
                        coord.x = max_extents.x;
                    }

                    if coord.y > max_extents.y {
                        coord.y = 1;
                    }
                    if coord.y < 1 {
                        coord.y = max_extents.y;
                    }

                    assert!(coord.x > 0);
                    assert!(coord.y > 0);
                    assert!(coord.x < max_extents.x + 1);
                    assert!(coord.y < max_extents.y + 1);
                }

                blizzard_cache.insert(next.count + 1, blizzards.clone());
                blizzards
            }
        };

        next.count += 1;

        let mut future = VecDeque::new();

        for dir in [
            Coord::new(1, 0),
            Coord::new(0, 1),
            Coord::new(-1, 0),
            Coord::new(0, -1),
            Coord::new(0, 0),
        ] {
            let mut new_node = next.clone();
            new_node.coord = new_node.coord + dir;

            if map.get(&new_node.coord) == Some(&Tile::Empty) {
                if blizzards
                    .iter()
                    .find(|(v, _)| *v == new_node.coord)
                    .is_none()
                    && explored.insert(new_node.clone())
                {
                    future.push_back(new_node);
                }
            }
        }
        exploration_queue.append(&mut future);
    }
    println!("{}", dest.count);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Blizzard {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum SearchState {
    ToEnd,
    ToStart,
    ToFinish,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct SearchNode {
    coord: Coord,
    count: u32,
    state: SearchState,
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
