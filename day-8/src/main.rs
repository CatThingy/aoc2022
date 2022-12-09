use std::collections::HashMap;

fn main() {
    let input = std::io::stdin();

    let mut trees = HashMap::<(u32, u32), u8>::new();
    let mut size = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };
        size.1 = size.1.max(y as u32);

        for (x, char) in line.chars().enumerate() {
            if char.is_numeric() {
                size.0 = size.0.max(x as u32);
                trees.insert((x as u32, y as u32), char as u8 - '0' as u8);
            }
        }
    }
    let mut max_score = 0;
    for (tree, _) in trees.iter() {
        max_score = max_score.max(get_scenic_score(&trees, tree, &size));
    }

    println!("{max_score}");
}

fn get_scenic_score(
    trees: &HashMap<(u32, u32), u8>,
    position: &(u32, u32),
    size: &(u32, u32),
) -> u32 {
    let this_tree = trees.get(position).unwrap();
    let mut scenic_score = 1_u32;

    // up
    let mut count = 0;
    for y in (0..position.1).rev() {
        count += 1;
        let next_tree = trees.get(&(position.0 as u32, y as u32)).unwrap();

        if next_tree >= this_tree {
            break;
        }
    }

    scenic_score *= count;

    count = 0;

    // down
    for y in (position.1 + 1)..=size.1 {
        count += 1;
        let next_tree = trees.get(&(position.0 as u32, y as u32)).unwrap();

        if next_tree >= this_tree {
            break;
        }
    }

    scenic_score *= count;

    count = 0;

    for x in (0..position.0).rev() {
        count += 1;
        let next_tree = trees.get(&(x as u32, position.1 as u32)).unwrap();

        if next_tree >= this_tree {
            break;
        }
    }

    scenic_score *= count;

    count = 0;

    for x in (position.0 + 1)..=size.0 {
        count += 1;
        let next_tree = trees.get(&(x as u32, position.1 as u32)).unwrap();
        if next_tree >= this_tree {
            break;
        }
    }

    scenic_score *= count;

    return scenic_score;
}
