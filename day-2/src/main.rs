fn main() {
    let input = std::io::stdin();
    let mut total_score = 0_u32;
    for line in input.lines() {
        let Ok(line) = line else { break; };

        let values: Vec<&str> = line.split(" ").collect();
        if values.len() != 2 {
            break;
        }
        let opponent = values[0];
        let player = values[1];

        match player {
            // Rock
            "X" => {
                total_score += 1;

                total_score += match opponent {
                    // Rock
                    "A" => 3,
                    // Paper
                    "B" => 0,
                    // Scissors
                    "C" => 6,
                    _ => unreachable!(),
                }
            }
            // Paper
            "Y" => {
                total_score += 2;
                total_score += match opponent {
                    // Rock
                    "A" => 6,
                    // Paper
                    "B" => 3,
                    // Scissors
                    "C" => 0,
                    _ => unreachable!(),
                }
            }
            // Scissors
            "Z" => {
                total_score += 3;
                total_score += match opponent {
                    // Rock
                    "A" => 0,
                    // Paper
                    "B" => 6,
                    // Scissors
                    "C" => 3,
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{total_score}");
}
