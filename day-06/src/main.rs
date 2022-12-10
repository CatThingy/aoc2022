use std::collections::VecDeque;

fn main() {
    let input = std::io::stdin();
    let mut buf = String::new();
    let mut count = 14;
    let Ok(_) = input.read_line(&mut buf) else { return; };
    let chars = buf.chars();

    let mut current = chars.take(14).collect::<VecDeque<char>>();

    for char in buf.chars().skip(14) {
        if confirm_uniqueness(&current) {
            break;
        }
        current.pop_front();
        current.push_back(char);
        count += 1;
    }
    println!("{}", count);
}

fn confirm_uniqueness(input: &VecDeque<char>) -> bool {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }
            if input[i] == input[j] {
                return false;
            }
        }
    }
    return true;
}
