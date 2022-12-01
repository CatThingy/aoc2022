fn main() {
    let input = std::io::stdin();

    let mut greatest_total = 0_u32;
    let mut acc = 0_u32;

    for line in input.lines() {
        let Ok(line) = line else { break; };
        if let Ok(num) = line.parse::<u32>() {
            acc += num;
        } else {
            if acc > greatest_total {
                greatest_total = acc;
            }
            acc = 0;
        }
    }

    if acc > greatest_total {
        greatest_total = acc;
    }

    println!("{greatest_total}");
}
