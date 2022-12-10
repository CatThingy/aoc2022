const WIDTH: i32 = 40;
const HEIGHT: i32 = 6;
fn main() {
    let input = std::io::stdin();

    let mut count = 0;
    let mut register = 1;

    let mut crt_display = [' '; WIDTH as usize * HEIGHT as usize];

    for line in input.lines() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };

        let command = line.split_whitespace().collect::<Vec<_>>();

        match command[0] {
            "addx" => {
                let amount: i32 = command[1].parse().unwrap();
                cycle_update(&register, &count, &mut crt_display);
                count += 1;

                cycle_update(&register, &count, &mut crt_display);
                count += 1;
                register += amount;
            }
            _ => {
                cycle_update(&register, &count, &mut crt_display);
                count += 1;
            }
        }
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", crt_display[(y * WIDTH + x) as usize])
        }
        print!("\n")
    }
}

fn cycle_update(
    register: &i32,
    cycle: &i32,
    display: &mut [char; WIDTH as usize * HEIGHT as usize],
) {
    if (register % WIDTH - cycle % WIDTH).abs() <= 1 {
        display[*cycle as usize] = '█'
    }
}
