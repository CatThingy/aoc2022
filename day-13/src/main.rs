fn main() {
    let mut input = std::io::stdin().lines();
    let mut index = 1;

    let mut score = 0;
    while let Some(Ok(line)) = input.next() {
        dbg!(&line);
        // dbg!(parse_packet_data(&line).unwrap());
        dbg!(index);
        let packet_1 = parse_packet_data(&line).unwrap();
        let packet_2 = parse_packet_data(&input.next().unwrap().unwrap()).unwrap();

        if packet_1.compare(&packet_2).unwrap_or(true) {
            println!("success: \n{packet_1:?}\n{packet_2:?}\n");
            score += index;
        } else {
            println!("failure: \n{packet_1:?}\n{packet_2:?}\n");
        }

        input.next();

        index += 1;
    }

    println!("{score}");
}

#[derive(Debug, PartialEq, Clone)]
enum PacketData {
    Int(u32),
    List(Vec<PacketData>),
}

impl PacketData {
    fn compare(&self, other: &Self) -> Option<bool> {
        println!("Comparing {self:?} and {other:?}!");
        match (self, other) {
            (PacketData::Int(s), PacketData::Int(o)) => {
                if s != o {
                    Some(s < o)
                } else {
                    None
                }
            }
            (PacketData::Int(_), PacketData::List(_)) => {
                PacketData::List(vec![self.clone()]).compare(other)
            }
            (PacketData::List(_), PacketData::Int(_)) => {
                self.compare(&PacketData::List(vec![other.clone()]))
            }
            (PacketData::List(s), PacketData::List(o)) => {
                for (s, o) in s.iter().zip(o.iter()) {
                    match s.compare(o) {
                        Some(t) => return Some(t),
                        _ => {}
                    }
                }
                    if s.len() != o.len() {
                        Some(s.len() < o.len())
                    } else {
                        None
                    }
            }
        }
    }
}

fn parse_packet_data(input: &str) -> Option<PacketData> {
    let trimmed = if input.starts_with('[') && input.ends_with(']') {
        &input[1..(input.len() - 1)]
    } else {
        if input.len() == 0 {
            return None;
        }
        let int = input.parse::<u32>();
        match int {
            Ok(v) => return Some(PacketData::Int(v)),
            _ => return None,
        }
    };

    let mut packets = Vec::<String>::new();

    let mut split = trimmed.split(',');

    while let Some(substr) = split.next() {
        if substr.starts_with('[') {
            if substr.ends_with(']')
                && substr.chars().filter(|v| *v == '[').count() == 1
                && substr.chars().filter(|v| *v == ']').count() == 1
            {
                packets.push(substr.to_owned());
            } else {
                let mut total = String::new();

                let mut target = substr;

                let mut depth = target.chars().filter(|v| *v == '[').count()
                    - target.chars().filter(|v| *v == ']').count();

                while depth != 0 {
                    total += target;
                    total += ",";
                    target = split.next().unwrap();

                    if target.starts_with('[') {
                        depth += target.chars().filter(|v| *v == '[').count();
                    }

                    if target.ends_with(']') {
                        depth -= target.chars().filter(|v| *v == ']').count();
                    }
                }
                total += target;
                packets.push(total.to_owned());
            }
        } else {
            packets.push(substr.to_owned());
        }
    }

    let result = packets
        .into_iter()
        .map(|v| parse_packet_data(&v))
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .collect();

    Some(PacketData::List(result))
}
