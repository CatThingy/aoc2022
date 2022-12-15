use std::collections::{HashMap, HashSet};

const SCAN_LINE: i32 = 2000000;

fn main() {
    let input = std::io::stdin();

    let mut sensors = HashMap::<Coord, u32>::new();
    let mut beacons = HashSet::<Coord>::new();

    let mut min_x: i32 = i32::MAX;
    let mut max_x: i32 = i32::MIN;

    let mut max_dist = 0;

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

        min_x = min_x.min(sensor_coords.x);
        max_x = max_x.max(sensor_coords.x);

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

        let dist = sensor_coords.distance(&beacon_coords);
        max_dist = max_dist.max(dist);

        sensors.insert(sensor_coords, sensor_coords.distance(&beacon_coords));
        beacons.insert(beacon_coords);
    }

    min_x -= max_dist as i32;
    max_x += max_dist as i32;

    let mut counter = 0;

    for x in min_x..max_x {
        let test_coord = Coord::new(x, SCAN_LINE);
        if beacons.contains(&test_coord) {
            continue;
        }
        for (sensor, dist) in sensors.iter() {
            if test_coord.distance(sensor) <= *dist {
                counter += 1;
                break;
            }
        }
    }

    dbg!(counter);
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

// impl Add for Coord {
//     type Output = Coord;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         Self {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }
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
