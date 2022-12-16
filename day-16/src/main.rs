use std::collections::{HashMap, HashSet};
fn main() {
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
        open: HashSet::new(),
        flow_rate: 0,
        pressure: 0,
        agents: [(Action::Open, "AA"), (Action::Open, "AA")],
    });

    let mut counter = 0;

    while counter < 26 {
        let mut new_nodes = Vec::<Node>::new();
        for path in &mut paths {
            let fresh_clone = path.clone();

            path.agents[0].0 = Action::Open;
            path.agents[1].0 = Action::Open;

            for next_valve_0 in &tunnels[path.agents[0].1] {
                if valves[path.agents[1].1] != 0 && !path.open.contains(path.agents[1].1) {
                    let mut node = fresh_clone.clone();
                    node.agents[0].0 = Action::Move(next_valve_0);
                    node.agents[1].0 = Action::Open;
                    new_nodes.push(node);
                }
            }

            for next_valve_1 in &tunnels[path.agents[1].1] {
                if valves[path.agents[0].1] != 0 && !path.open.contains(path.agents[0].1) {
                    let mut node = fresh_clone.clone();
                    node.agents[0].0 = Action::Open;
                    node.agents[1].0 = Action::Move(next_valve_1);
                    new_nodes.push(node);
                }
            }

            for next_valve_0 in &tunnels[path.agents[0].1] {
                for next_valve_1 in &tunnels[path.agents[1].1] {
                    let mut node = fresh_clone.clone();
                    node.agents[0].0 = Action::Move(next_valve_0);
                    node.agents[1].0 = Action::Move(next_valve_1);
                    new_nodes.push(node);
                }
            }

            if valves[path.agents[0].1] == 0
                || path.open.contains(path.agents[0].1)
                || valves[path.agents[1].1] == 0
                || path.open.contains(path.agents[1].1)
            {
                *path = new_nodes.pop().unwrap();
            }
        }

        paths.append(&mut new_nodes);
        // valves[path.agents[0].1] != 0
        //                 && !path.open.contains(path.agents[0].1)
        //                 && valves[path.agents[1].1] != 0
        //                 && !path.open.contains(path.agents[1].1)

        for path in &mut paths {
            path.pressure += path.flow_rate;
            for (action, ref mut location) in &mut path.agents {
                match action {
                    Action::Open => {
                        if path.open.insert(location) {
                            path.flow_rate += valves[*location];
                        }
                    }
                    Action::Move(dest) => {
                        *location = dest;
                    }
                }
            }
        }
        counter += 1;

        // Beam search
        if paths.len() > 10_000 {
            paths.sort_unstable_by_key(|v| u32::MAX - v.pressure);
            paths.dedup_by(|a, b| a == b);
            paths.drain(10_000..);
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

#[derive(Clone, Debug, PartialEq)]
struct Node<'a> {
    open: HashSet<&'a str>,
    flow_rate: u32,
    pressure: u32,
    agents: [(Action<'a>, &'a str); 2],
}
