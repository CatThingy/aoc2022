fn main() {
    let input = std::io::stdin();

    let mut blueprints = Vec::<Blueprint>::new();

    for line in input.lines() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };

        let line = line.split_whitespace().collect::<Vec<_>>();

        let ore_robot = line[6].parse::<u8>().unwrap();

        let clay_robot = line[12].parse::<u8>().unwrap();
        let obsidian_robot = (
            line[18].parse::<u8>().unwrap(),
            line[21].parse::<u8>().unwrap(),
        );
        let geode_robot = (
            line[27].parse::<u8>().unwrap(),
            line[30].parse::<u8>().unwrap(),
        );

        blueprints.push(Blueprint {
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        });
    }

    let mut score = Vec::<(usize, u32)>::new();

    for (i, blueprint) in blueprints.iter().enumerate() {
        let mut search_nodes = Vec::<Node>::new();

        search_nodes.push(Node {
            robots: [1, 0, 0, 0],
            resources: [0; 4],
            build: None,
        });

        for j in 0..24 {
            let mut new_nodes = Vec::<Node>::new();
            for path in &mut search_nodes {
                path.build = None;

                if path.resources[0] >= blueprint.ore_robot {
                    let mut new = path.clone();
                    new.build = Some(0);
                    new.resources[0] -= blueprint.ore_robot;
                    new_nodes.push(new);
                }
                if path.resources[0] >= blueprint.clay_robot {
                    let mut new = path.clone();
                    new.build = Some(1);
                    new.resources[0] -= blueprint.clay_robot;
                    new_nodes.push(new);
                }
                if path.resources[0] >= blueprint.obsidian_robot.0
                    && path.resources[1] >= blueprint.obsidian_robot.1
                {
                    let mut new = path.clone();
                    new.build = Some(2);
                    new.resources[0] -= blueprint.obsidian_robot.0;
                    new.resources[1] -= blueprint.obsidian_robot.1;
                    new_nodes.push(new);
                }
                if path.resources[0] >= blueprint.geode_robot.0
                    && path.resources[2] >= blueprint.geode_robot.1
                {
                    let mut new = path.clone();
                    new.build = Some(3);
                    new.resources[0] -= blueprint.geode_robot.0;
                    new.resources[2] -= blueprint.geode_robot.1;
                    new_nodes.push(new);
                }
            }
            search_nodes.append(&mut new_nodes);

            for path in &mut search_nodes {
                for i in 0..4 {
                    path.resources[i] += path.robots[i];
                }
                match path.build {
                    Some(v) => path.robots[v] += 1,
                    None => {}
                }
            }

            if search_nodes.len() > 4_000_000 {
                search_nodes.sort_unstable_by_key(|v| {
                    u32::MAX
                        - (v.resources[2] as u32 / 2
                            + v.robots[3] as u32 * blueprint.geode_robot.1 as u32)
                });
                search_nodes.drain(4_000_000..);
            }

            dbg!((j, search_nodes.len()));
        }

        search_nodes.sort_unstable_by_key(|v| u32::MAX - v.resources[3] as u32);
        score.push((i, search_nodes[0].resources[3] as u32));
    }

    dbg!(&score);
    let score = score
        .into_iter()
        .fold(0, |acc, v| acc + (v.0 + 1) * v.1 as usize);
    dbg!(score);
}

struct Blueprint {
    ore_robot: u8,
    clay_robot: u8,
    obsidian_robot: (u8, u8),
    geode_robot: (u8, u8),
}

#[derive(Clone)]
struct Node {
    robots: [u8; 4],
    resources: [u8; 4],
    build: Option<usize>,
}
