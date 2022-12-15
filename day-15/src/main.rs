use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

const SEARCH_SPACE: i32 = 4000000;

fn main() {
    let input = std::io::stdin();

    let mut sensors = HashMap::<Coord, u32>::new();
    let mut beacons = HashSet::<Coord>::new();

    let mut search = HashSet::<Coord>::new();

    for line in input.lines() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };

        let (sensor_str, beacon_str) = line.split_once(": ").unwrap();

        let sensor_coords = sensor_str
            .split(" ")
            .skip(2)
            .map(|v| {
                v.trim_start_matches("x=")
                    .trim_start_matches("y=")
                    .trim_end_matches(",")
                    .parse::<i32>()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let sensor_coords = Coord::new(sensor_coords[0], sensor_coords[1]);

        let beacon_coords = beacon_str
            .split(" ")
            .skip(4)
            .map(|v| {
                v.trim_start_matches("x=")
                    .trim_start_matches("y=")
                    .trim_end_matches(",")
                    .parse::<i32>()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let beacon_coords = Coord::new(beacon_coords[0], beacon_coords[1]);

        let dist = sensor_coords.distance(&beacon_coords) as i32 + 1;
        for x in (-dist)..=(dist) {
            if sensor_coords.x + x < 0 || sensor_coords.x + x > SEARCH_SPACE {
                continue;
            }
            let y_range = dist - x.abs();
            for y in [-y_range, y_range] {
                if sensor_coords.y + y < 0 || sensor_coords.y + y > SEARCH_SPACE {
                    continue;
                }
                search.insert(sensor_coords + Coord { x, y });
            }
        }

        sensors.insert(sensor_coords, sensor_coords.distance(&beacon_coords));
        beacons.insert(beacon_coords);
    }

    'a: {
        for coord in search {
            if coord.x < 0 || coord.x > SEARCH_SPACE || coord.y < 0 || coord.y > SEARCH_SPACE {
                continue;
            }
            let mut found = true;
            for (sensor, dist) in sensors.iter() {
                if sensor.distance(&coord) <= *dist {
                    found = false;
                    break;
                }
            }

            if found {
                println!("{}", coord.x as u64 * 4000000 + coord.y as u64);
                break 'a;
            }
        }
    }
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
    const fn distance(&self, other: &Coord) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
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
//
// impl AddAssign for Coord {
//     fn add_assign(&mut self, rhs: Self) {
//         *self = *self + rhs;
//     }
// }
//
// impl Sub for Coord {
//     type Output = Coord;
//
//     fn sub(self, rhs: Self) -> Self::Output {
//         Self {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//         }
//     }
// }
//
// impl SubAssign for Coord {
//     fn sub_assign(&mut self, rhs: Self) {
//         *self = *self - rhs;
//     }
// }
