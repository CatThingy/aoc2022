fn main() {
    let input = std::io::stdin();

    let mut acc = 0;
    for line in input.lines() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };

        // println!("{}", to_snafu(line.parse().unwrap()));
        acc += parse_snafu(&line);
    }
    println!("{}", to_snafu(acc as i64));
}

fn parse_snafu(input: &str) -> u64 {
    let mut acc = 0_u64;
    for char in input.chars() {
        let next = match char {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!(),
        };

        acc *= 5;
        acc = acc.saturating_add_signed(next);
    }

    acc
}

fn to_snafu(input: i64) -> String {
    let mut out = String::new();

    let mut acc = input;

    let mut largest_power = if input < 3 {
        0
    } else {
        let mut largest = 1;
        loop {
            if 2 * 5_i64.pow(largest) + 2 * 5_i64.pow(largest - 1) >= input {
                break;
            }
            largest += 1;
        }
        largest
    };

    loop {
        let mut next_digit = -2;
        loop {
            if largest_power == 0 {
                next_digit = acc;
                break;
            } else if next_digit * 5_i64.pow(largest_power)
                >= acc - (2 * 5_i64.pow(largest_power)) / 4
            {
                break;
            }
            next_digit += 1;
        }

        dbg!(next_digit);
        dbg!(next_digit * 5_i64.pow(largest_power));

        let digit = match next_digit {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        };
        out.push(digit);

        acc -= next_digit * 5_i64.pow(largest_power);
        dbg!(acc);
        if largest_power == 0 {
            break;
        }
        largest_power = largest_power.saturating_sub(1);
    }

    // println!("{largest_power}");

    out
}
