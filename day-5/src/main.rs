use std::collections::VecDeque;

fn main() {
    let input = std::io::stdin();
    let mut state = ParseState::Initial;

    let mut stack: [VecDeque<char>; 9] = std::array::from_fn(|_| VecDeque::new());

    for line in input.lines() {
        let Ok(line) = line else { break; };

        match state {
            ParseState::Initial => {
                if line.len() == 0 {
                    state = ParseState::Movement;
                    continue;
                }

                for (index, chunk) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                    let ident = chunk[1];

                    if !ident.is_ascii_alphabetic() {
                        continue;
                    }

                    stack[index].push_back(ident);
                }
            }
            ParseState::Movement => {
                if line.len() == 0 {
                    break;
                }

                let val = line.split_whitespace().collect::<Vec<&str>>();
                let count = val[1].parse::<usize>().unwrap();
                let target = val[3].parse::<usize>().unwrap() - 1;
                let dest = val[5].parse::<usize>().unwrap() - 1;

                let mut removed = Vec::with_capacity(count);
                for _ in 0..count {
                    removed.push(stack[target].pop_front().unwrap());
                }

                for removed in removed.iter().rev() {
                    stack[dest].push_front(*removed);
                }
            }
        }
    }

    for val in stack {
        print!("{}", val.front().unwrap());
    }
    print!("\n");
}

#[derive(PartialEq)]
enum ParseState {
    Initial,
    Movement,
}
