use std::{
    collections::{HashSet, VecDeque},
    ops::{Add, Sub},
};

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

    let mut internal_air = HashSet::<Coord>::new();

    let mut min_corner = Coord::new(i32::MAX, i32::MAX, i32::MAX);
    let mut max_corner = Coord::new(i32::MIN, i32::MIN, i32::MIN);

    for cube in &cubes {
        min_corner.x = min_corner.x.min(cube.x);
        min_corner.y = min_corner.y.min(cube.y);
        min_corner.z = min_corner.y.min(cube.z);

        max_corner.x = max_corner.x.max(cube.x);
        max_corner.y = max_corner.y.max(cube.y);
        max_corner.z = max_corner.z.max(cube.z);
    }

    let mut counter = 0;

    for cube in &cubes {
        for direction in DIRECTIONS {
            let search_begin = *cube + direction;

            if !cubes.contains(&search_begin)
                && !internal_air.contains(&search_begin)
                && floodfill_internal_air(
                    search_begin,
                    &min_corner,
                    &max_corner,
                    &cubes,
                    &mut internal_air,
                )
            {
                counter += 1;
            }
        }
    }
    dbg!(internal_air);

    println!("{counter}");
}

fn floodfill_internal_air(
    search_begin: Coord,
    min_corner: &Coord,
    max_corner: &Coord,
    cubes: &HashSet<Coord>,
    internal_air: &mut HashSet<Coord>,
) -> bool {
    let mut explored = HashSet::<Coord>::new();
    let mut exploration_queue = VecDeque::<Coord>::new();

    explored.insert(search_begin);
    exploration_queue.push_back(search_begin);

    let mut exposed_to_air = false;
    while let Some(next) = exploration_queue.pop_front() {
        if next.x < min_corner.x
            || next.y < min_corner.y
            || next.z < min_corner.z
            || next.x > max_corner.x
            || next.y > max_corner.y
            || next.z > max_corner.z
        {
            exposed_to_air = true;
            break;
        }

        for direction in DIRECTIONS {
            let new_coord = next + direction;
            if !cubes.contains(&new_coord) && !explored.contains(&new_coord) {
                // if next == Coord::new(2, 2, 5) {
                //     // dbg!("AAAA");
                // }
                explored.insert(new_coord);
                exploration_queue.push_back(new_coord);
            }
        }
    }
    if !exposed_to_air {
        dbg!("AAA");
        for key in explored.into_iter() {
            internal_air.insert(key);
        }
    }
    return exposed_to_air;
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
