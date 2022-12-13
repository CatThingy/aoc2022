fn main() {
    let mut input = std::io::stdin().lines();

    let mut packets = Vec::<PacketData>::new();
    while let Some(Ok(line)) = input.next() {
        if let Some(packet) = parse_packet_data(&line) {
            packets.push(packet);
        }
    }
    packets.push(PacketData::List(vec![PacketData::List(vec![
        PacketData::Int(2),
    ])]));
    packets.push(PacketData::List(vec![PacketData::List(vec![
        PacketData::Int(6),
    ])]));
    packets.sort();

    let index_1 = packets
        .binary_search(&PacketData::List(vec![PacketData::List(vec![
            PacketData::Int(2),
        ])]))
        .unwrap();

    let index_2 = packets
        .binary_search(&PacketData::List(vec![PacketData::List(vec![
            PacketData::Int(6),
        ])]))
        .unwrap();

    println!("{}", (index_1 + 1) * (index_2 + 1));
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum PacketData {
    Int(u32),
    List(Vec<PacketData>),
}
impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}
impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (PacketData::Int(s), PacketData::Int(o)) => {
                if s != o {
                    s.partial_cmp(o)
                } else {
                    None
                }
            }
            (PacketData::Int(_), PacketData::List(_)) => {
                PacketData::List(vec![self.clone()]).partial_cmp(other)
            }
            (PacketData::List(_), PacketData::Int(_)) => {
                self.partial_cmp(&PacketData::List(vec![other.clone()]))
            }
            (PacketData::List(s), PacketData::List(o)) => {
                for (s, o) in s.iter().zip(o.iter()) {
                    match s.partial_cmp(o) {
                        Some(t) => return Some(t),
                        _ => {}
                    }
                }
                if s.len() != o.len() {
                    s.len().partial_cmp(&o.len())
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
