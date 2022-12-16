use std::collections::{HashMap, HashSet};
fn main() {
    dbg!(std::mem::size_of::<Node>());
    let input = std::io::stdin();

    let mut valves = HashMap::<String, u32>::new();
    let mut tunnels = HashMap::<String, Vec<String>>::new();

    for line in input.lines() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };

        let line = line.split(" ").collect::<Vec<_>>();

        let valve_name = line[1];

        let flow_rate = line[4]
            .trim_start_matches("rate=")
            .trim_end_matches(";")
            .parse::<u32>()
            .unwrap();

        valves.insert(valve_name.to_owned(), flow_rate);

        let connected = line[9..].iter().map(|v| v.trim_end_matches(",").to_owned());

        tunnels.insert(valve_name.to_owned(), connected.collect());
    }

    let mut paths = Vec::<Node>::new();

    paths.push(Node {
        next_action: Action::Open,
        open: HashSet::new(),
        flow_rate: 0,
        pressure: 0,
        location: "AA",
    });

    let mut counter = 0;

    while counter < 30 {
        let mut new_nodes = Vec::<Node>::new();
        for path in &mut paths {
            let fresh_clone = path.clone();
            let mut first = true;
            if valves[path.location] != 0 && !path.open.contains(path.location) {
                path.next_action = Action::Open;
                first = false;
            }

            for next_valve in &tunnels[path.location] {
                if first {
                    path.next_action = Action::Move(next_valve);
                    first = false;
                } else {
                    let mut node = fresh_clone.clone();
                    node.next_action = Action::Move(next_valve);
                    new_nodes.push(node);
                }
            }
        }

        paths.append(&mut new_nodes);

        for path in &mut paths {
            path.pressure += path.flow_rate;
            match path.next_action {
                Action::Open => {
                    path.open.insert(path.location);
                    path.flow_rate += valves[path.location];
                }
                Action::Move(dest) => {
                    path.location = dest;
                }
            }
        }
        counter += 1;

        // Beam search
        if paths.len() > 1_000_000 {
            paths.sort_unstable_by_key(|v| u32::MAX - v.pressure);
            paths.drain(1_000_000..);
        }

        dbg!(counter, paths.len());
    }

    paths.sort_unstable_by_key(|v| u32::MAX - v.pressure);
    dbg!(&paths[0]);
}

#[derive(Clone, PartialEq, Debug)]
enum Action<'a> {
    Open,
    Move(&'a str),
}

#[derive(Clone, Debug)]
struct Node<'a> {
    next_action: Action<'a>,
    open: HashSet<&'a str>,
    flow_rate: u32,
    pressure: u32,
    location: &'a str,
}
