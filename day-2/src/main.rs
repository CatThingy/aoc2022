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
            // Lose
            "X" => {
                total_score += match opponent {
                    // Rock; choose scissors
                    "A" => 0 + 3,
                    // Paper; choose rock
                    "B" => 0 + 1,
                    // Scissors; choose paper
                    "C" => 0 + 2,
                    _ => unreachable!(),
                }
            }
            // Draw
            "Y" => {
                total_score += match opponent {
                    // Rock; choose rock
                    "A" => 3 + 1,
                    // Paper; choose paper
                    "B" => 3 + 2,
                    // Scissors; choose scissors
                    "C" => 3 + 3,
                    _ => unreachable!(),
                }
            }
            // Win
            "Z" => {
                total_score += match opponent {
                    // Rock; choose paper
                    "A" => 6 + 2,
                    // Paper; choose scissors
                    "B" => 6 + 3,
                    // Scissors; choose rock
                    "C" => 6 + 1,
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{total_score}");
}
