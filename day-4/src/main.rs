fn main() {
    let input = std::io::stdin();
    let mut total = 0_u32;

    for line in input.lines() {
        let Ok(line) = line else { break; };
        let Some(pairs) = line.split_once(',') else { break };

        let range_1 = pairs
            .0
            .split_once('-')
            .map(|v| (v.0.parse::<u32>().unwrap(), v.1.parse::<u32>().unwrap()))
            .unwrap();
        let range_2 = pairs
            .1
            .split_once('-')
            .map(|v| (v.0.parse::<u32>().unwrap(), v.1.parse::<u32>().unwrap()))
            .unwrap();

        if (range_1.0 >= range_2.0 && range_1.1 <= range_2.1)
            || (range_1.0 <= range_2.0 && range_1.1 >= range_2.1)
        {
            total += 1;
        }
    }
    println!("{total}");
}
