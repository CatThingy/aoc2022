use std::collections::HashSet;

fn main() {
    let input = std::io::stdin();
    let mut total_priority = 0_u32;

    for line in input.lines() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        }
        let half_1: HashSet<char> = HashSet::from_iter(&mut line[..line.len() / 2].chars());
        for char in &mut line[line.len() / 2..].chars() {
            if half_1.contains(&char) {
                total_priority += char_to_priority(char) as u32;
                break;
            }
        }
    }

    dbg!(total_priority);
}

fn char_to_priority(char: char) -> u8 {
    dbg!(char, char as u8);
    match char {
        'a'..='z' => {
            char as u8 - 'a' as u8 + 1
        }
        'A'..='Z' => {
            char as u8 - 'A' as u8 + 27
        }
        _ => unreachable!(),
    }
}
