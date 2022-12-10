const INTERVAL: i32 = 40;
const START_POINT: i32 = 20;
fn main() {
    let input = std::io::stdin();

    let mut count = 0;
    let mut register = 1;
    let mut total = 0;

    for line in input.lines() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };

        let command = line.split_whitespace().collect::<Vec<_>>();

        match command[0] {
            "addx" => {
                let amount: i32 = command[1].parse().unwrap();
                count += 1;

                if (count - START_POINT) % INTERVAL == 0 {
                    total += count * register;
                }
                count += 1;
                if (count - START_POINT) % INTERVAL == 0 {
                    total += count * register;
                }
                register += amount;
            }
            _ => {
                count += 1;

                if (count - START_POINT) % INTERVAL == 0 {
                    dbg!((count, register, count * register));
                    total += count * register;
                }
            }
        }
    }

    dbg!(total);
}
