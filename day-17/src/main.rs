use std::{
    collections::HashSet,
    hash::Hasher,
    ops::{Add, AddAssign, Sub, SubAssign},
};

static SHAPES: ([Coord; 4], [Coord; 5], [Coord; 5], [Coord; 4], [Coord; 4]) = (
    [
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(3, 0),
    ],
    [
        Coord::new(0, 1),
        Coord::new(1, 0),
        Coord::new(1, 1),
        Coord::new(1, 2),
        Coord::new(2, 1),
    ],
    [
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(2, 1),
        Coord::new(2, 2),
    ],
    [
        Coord::new(0, 0),
        Coord::new(0, 1),
        Coord::new(0, 2),
        Coord::new(0, 3),
    ],
    [
        Coord::new(0, 0),
        Coord::new(0, 1),
        Coord::new(1, 0),
        Coord::new(1, 1),
    ],
);

const DIRECTIONS: [Coord; 2] = [Coord::new(1, 0), Coord::new(-1, 0)];

fn main() {
    let shapes: [&[Coord]; 5] = [&SHAPES.0, &SHAPES.1, &SHAPES.2, &SHAPES.3, &SHAPES.4];
    let input = std::io::stdin();

    let mut pattern = vec![];

    let mut offset = Coord::new(2, 3);

    let mut occupied = HashSet::<Coord>::new();

    for line in input.lines() {
        let Ok(line) = line else { break; };
        for char in line.chars() {
            pattern.push(match char {
                '>' => 0,
                '<' => 1,
                _ => unreachable!(),
            });
        }
    }

    let mut shape_index = 0;
    let mut pattern_index = 0;

    for i in 0..2022 {
        let mut shape = shapes[shape_index].to_vec();
        shape_index = (shape_index + 1) % shapes.len();

        for coord in &mut shape {
            *coord += offset;
        }

        loop {
            let wind = DIRECTIONS[pattern[pattern_index]];
            pattern_index = (pattern_index + 1) % pattern.len();
            let mut can_move = true;
            for coord in &shape {
                let next_pos = *coord + wind;
                if next_pos.x < 0 || next_pos.x > 6 || occupied.contains(&next_pos) {
                    can_move = false;
                    break;
                }
            }

            if can_move {
                for coord in &mut shape {
                    *coord += wind;
                }
            }

            let mut touched = false;
            for coord in &shape {
                let next_pos = *coord + Coord::new(0, -1);
                if next_pos.y < 0 || occupied.contains(&next_pos) {
                    touched = true;
                    break;
                }
            }

            if touched {
                let mut max_y = offset.y - 4;
                for coord in shape {
                    max_y = max_y.max(coord.y);
                    assert!(occupied.insert(coord));
                }

                dbg!(max_y);
                offset = Coord::new(2, 3 + max_y + 1);
                break;
            } else {
                for coord in &mut shape {
                    *coord += Coord::new(0, -1);
                }
            }
        }
    }

    let tallest_point = occupied.iter().max_by_key(|v| v.y).unwrap().y + 1;

    // print_stack(&occupied, tallest_point.unwrap().y, 7);
    println!("{tallest_point}");
}

fn print_stack(stack: &HashSet<Coord>, height: i32, width: i32) {
    for y in (0..=height).rev() {
        for x in 0..=width {
            let output = match stack.contains(&Coord::new(x, y)) {
                true => "#",
                false => ".",
            };

            print!("{output}");
        }
        print!("\n");
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
