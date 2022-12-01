use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let input = std::io::stdin();

    let mut greatest = BinaryHeap::from([Reverse(0), Reverse(0), Reverse(0)]);

    let mut acc = 0_u32;

    for line in input.lines() {
        let Ok(line) = line else { break; };
        if let Ok(num) = line.parse::<u32>() {
            acc += num;
        } else {
            if acc > greatest.peek().unwrap().0 {
                greatest.pop();
                greatest.push(Reverse(acc));
            }
            acc = 0;
        }
    }

    if acc > greatest.peek().unwrap().0 {
        greatest.pop();
        greatest.push(Reverse(acc));
    }

    let total = greatest.into_vec().iter().fold(0, |a, v| a + v.0);

    println!("{total}");
}
