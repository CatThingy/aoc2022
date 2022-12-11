use std::cell::RefCell;

fn main() {
    let mut input = std::io::stdin().lines();

    let mut monkeys = Vec::<Monkey>::new();

    while let Some(Ok(_)) = input.next() {
        let items = input
            .next()
            .unwrap()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let next = input.next().unwrap().unwrap();
        let operation = next.split_once(": ").unwrap().1;

        let next = input.next().unwrap().unwrap();
        let test = next.split_once(": ").unwrap().1;

        let next = input.next().unwrap().unwrap();
        let true_eff = next.split_once(": ").unwrap().1;

        let next = input.next().unwrap().unwrap();
        let false_eff = next.split_once(": ").unwrap().1;

        let operation = operation_from_string(operation);

        let test = test_from_str(test, true_eff, false_eff);

        monkeys.push(Monkey {
            items: RefCell::new(items),
            operation,
            test,
        });

        input.next();
    }

    let mut inspection_counts = Vec::<u32>::with_capacity(monkeys.len());

    for _ in 0..monkeys.len() {
        inspection_counts.push(0);
    }

    for _ in 0..20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            for mut item in monkey.items.take().drain(..) {
                item = (monkey.operation)(item) / 3;

                inspection_counts[i] += 1;

                let throw_target = (monkey.test)(item);

                monkeys[throw_target].items.borrow_mut().push(item);
            }
        }
    }

    dbg!(inspection_counts);
}

struct Monkey {
    items: RefCell<Vec<u32>>,
    operation: Box<dyn Fn(u32) -> u32>,
    test: Box<dyn Fn(u32) -> usize>,
}

fn operation_from_string(operation: &str) -> Box<dyn Fn(u32) -> u32> {
    let operation = operation.split_whitespace().collect::<Vec<_>>();

    let second_operand = operation[4].parse::<u32>();

    match operation[3] {
        "+" => {
            if let Ok(op) = second_operand {
                Box::new(move |v: u32| v + op)
            } else {
                Box::new(|v: u32| v + v)
            }
        }
        "*" => {
            if let Ok(op) = second_operand {
                Box::new(move |v: u32| v * op)
            } else {
                Box::new(|v: u32| v * v)
            }
        }
        _ => unreachable!(),
    }
}

fn test_from_str(test: &str, true_eff: &str, false_eff: &str) -> Box<dyn Fn(u32) -> usize> {
    let dividend = test
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let true_eff = true_eff
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let false_eff = false_eff
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    Box::new(move |v: u32| {
        if v % dividend == 0 {
            true_eff
        } else {
            false_eff
        }
    })
}
