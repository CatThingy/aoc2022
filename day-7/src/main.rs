use std::{collections::HashMap, rc::Rc};

fn main() {
    let input = std::io::stdin();

    let mut traversal_tree: Vec<Rc<Node>> =
        vec![Rc::new(Node::Directory("/".to_string(), vec![], None))];

    // parsing time
    for line in input.lines().skip(1) {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };

        if line.starts_with('$') {
            let cmd = line.split_whitespace().skip(1).collect::<Vec<&str>>();

            match cmd[0] {
                "cd" => {
                    if cmd[1] != ".." {
                        let prev_node = traversal_tree.last().unwrap();
                        match prev_node.as_ref() {
                            Node::File(_, _) => unreachable!(),
                            Node::Directory(_, vec, _) => {
                                let next = vec.iter().find(|v| match v.as_ref() {
                                    Node::Directory(name, _, _) if name == cmd[1] => true,
                                    _ => false,
                                });
                                traversal_tree.push(Rc::clone(next.unwrap()));
                            }
                        }
                    } else {
                        traversal_tree.pop();
                    }
                }
                "ls" => {}
                _ => unreachable!(),
            }
        } else {
            let cmd = line.split_whitespace().collect::<Vec<&str>>();
            let node = match cmd[0] {
                "dir" => Rc::new(Node::Directory(cmd[1].to_string(), vec![], None)),
                num => Rc::new(Node::File(cmd[1].to_string(), num.parse::<u32>().unwrap())),
            };

            let prev_node = traversal_tree.last_mut().unwrap();
            unsafe {
                let prev_node = Rc::as_ptr(prev_node) as *mut Node;
                match &mut *prev_node {
                    Node::File(_, _) => unreachable!(),
                    Node::Directory(_, vec, _) => {
                        let contains = vec
                            .iter()
                            .find(|v| match v.as_ref() {
                                Node::File(name, _) => name == cmd[1],
                                Node::Directory(name, _, _) => name == cmd[1],
                            })
                            .is_some();
                        if !contains {
                            vec.push(node);
                        }
                    }
                }
            }
        }
    }

    let mut root = Rc::clone(&traversal_tree[0]);
    drop(traversal_tree);

    let mut sizes = HashMap::new();
    get_size(Rc::get_mut(&mut root).unwrap());
    get_directory_sizes(&root, &mut sizes, "");

    dbg!(&sizes);

    let size_total = sizes
        .iter()
        .filter(|v| v.1 <= &100000)
        .fold(0_u32, |a, v| a + dbg!(v.1));

    dbg!(size_total);
}

#[derive(Debug)]
enum Node {
    File(String, u32),
    Directory(String, Vec<Rc<Node>>, Option<u32>),
}

fn get_size(node: &mut Node) -> u32 {
    match node {
        Node::File(_, size) => *size,
        Node::Directory(_, vec, ref mut computed_size) => {
            let mut size: u32;
            if let Some(computed_size) = computed_size {
                size = *computed_size;
            } else {
                size = 0;

                for node in vec {
                    unsafe {
                        let node = Rc::as_ptr(node) as *mut Node;
                        size += get_size(&mut *node);
                    }
                }
            }

            *computed_size = Some(size);

            return size;
        }
    }
}

fn get_directory_sizes(node: &Node, sizes: &mut HashMap<String, u32>, prefix: &str) {
    match node {
        Node::File(_, _) => (),
        Node::Directory(name, vec, computed_size) => {
            sizes.insert(prefix.to_string() + name, computed_size.unwrap());
            for node in vec {
                let mut new_prefix = prefix.to_string();
                new_prefix.push_str(name);
                get_directory_sizes(node, sizes, &new_prefix);
            }
        }
    }
}
