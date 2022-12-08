use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::io::stdin();

    let mut trees = HashMap::<(u32, u32), u8>::new();
    let mut size = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };
        size.1 = size.1.max(y);

        for (x, char) in line.chars().enumerate() {
            if char.is_numeric() {
                size.0 = size.0.max(x);
                trees.insert((x as u32, y as u32), char as u8 - '0' as u8);
            }
        }
    }
    dbg!(size);

    let mut visible = HashSet::<(u32, u32)>::new();

    for x in 1..size.0 {
        let mut current_tree: Option<u8> = None;

        // top row
        for y in 0..size.1 {
            let next_tree = trees.get(&(x as u32, y as u32)).unwrap();

            match current_tree {
                Some(tree) if tree < *next_tree => {
                    current_tree = Some(tree.max(*next_tree));
                    visible.insert((x as u32, y as u32));
                }
                None => {
                    current_tree = Some(*next_tree);
                    visible.insert((x as u32, y as u32));
                }
                _ => (),
            }
        }

        current_tree = None;

        // bottom row
        for y in (1..=size.1).rev() {
            let next_tree = trees.get(&(x as u32, y as u32)).unwrap();
            match current_tree {
                Some(tree) if tree < *next_tree => {
                    current_tree = Some(tree.max(*next_tree));
                    visible.insert((x as u32, y as u32));
                }
                None => {
                    current_tree = Some(*next_tree);
                    visible.insert((x as u32, y as u32));
                }
                _ => (),
            }
        }
    }

    for y in 1..size.0 {
        let mut current_tree: Option<u8> = None;

        // left column
        for x in 0..size.1 {
            let next_tree = trees.get(&(x as u32, y as u32)).unwrap();
            match current_tree {
                Some(tree) if tree < *next_tree => {
                    current_tree = Some(tree.max(*next_tree));
                    visible.insert((x as u32, y as u32));
                }
                None => {
                    current_tree = Some(*next_tree);
                    visible.insert((x as u32, y as u32));
                }
                _ => (),
            }
        }
        current_tree = None;

        // right column
        for x in (1..=size.1).rev() {
            let next_tree = trees.get(&(x as u32, y as u32)).unwrap();
            match current_tree {
                Some(tree) if tree < *next_tree => {
                    current_tree = Some(tree.max(*next_tree));
                    visible.insert((x as u32, y as u32));
                }
                None => {
                    current_tree = Some(*next_tree);
                    visible.insert((x as u32, y as u32));
                }
                _ => (),
            }
        }
    }

    // didn't count corners, so +4
    println!("{:?}", visible.iter().len() + 4);
}
