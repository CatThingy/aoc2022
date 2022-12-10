fn main() {
    let input = std::io::stdin();
    let mut total_priority = 0_u32;

    let mut pack = 0;
    let mut possible = [0; 52];
    for line in input.lines() {
        let Ok(line) = line else { break; };

        pack += 1;

        for priority in line.chars().map(char_to_priority) {
            let priority = priority - 1;
            if possible[priority as usize] == pack - 1 {
                possible[priority as usize] += 1;
            }
        }

        if pack == 3 {
            total_priority += dbg!(possible.iter().position(|v| v == &3).unwrap() as u32 + 1);
            possible = [0; 52];
            pack = 0;
        }
    }

    dbg!(total_priority);
}

fn char_to_priority(char: char) -> u8 {
    match char {
        'a'..='z' => char as u8 - 'a' as u8 + 1,
        'A'..='Z' => char as u8 - 'A' as u8 + 27,
        _ => unreachable!(),
    }
}
