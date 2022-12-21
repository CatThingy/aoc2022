use std::{cell::RefCell, collections::HashMap};

fn main() {
    let input = std::io::stdin();

    let mut monkeys = HashMap::<String, RefCell<Monkey>>::new();

    for line in input.lines() {
        let Ok(line) = line else { break; };
        if line.len() == 0 {
            break;
        };

        let (name, data) = line.split_once(": ").unwrap();

        let data = data.split_whitespace().collect::<Vec<_>>();

        let monkey = if data.len() == 1 {
            Monkey::Value(data[0].parse().unwrap())
        } else {
            match data[1] {
                "+" => Monkey::Add(data[0].to_owned(), data[2].to_owned()),
                "-" => Monkey::Subtract(data[0].to_owned(), data[2].to_owned()),
                "*" => Monkey::Multiply(data[0].to_owned(), data[2].to_owned()),
                "/" => Monkey::Divide(data[0].to_owned(), data[2].to_owned()),
                _ => unreachable!(),
            }
        };
        monkeys.insert(name.to_owned(), RefCell::new(monkey));
    }

    eval_monkey(&monkeys, "root");

    dbg!(&monkeys["root"]);
}

#[derive(Debug)]
enum Monkey {
    Value(u64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

fn eval_monkey(monkeys: &HashMap<String, RefCell<Monkey>>, current: &str) {
    let current = &mut *monkeys.get(current).unwrap().borrow_mut();

    match current {
        Monkey::Value(_) => {
            return;
        }
        Monkey::Add(a, b) => {
            eval_monkey(monkeys, &a);
            eval_monkey(monkeys, &b);
            let Monkey::Value(a) = &*monkeys.get(a).unwrap().borrow() else { unreachable!() };
            let Monkey::Value(b) = &*monkeys.get(b).unwrap().borrow() else { unreachable!() };

            *current = Monkey::Value(a + b);
        }
        Monkey::Subtract(a, b) => {
            eval_monkey(monkeys, &a);
            eval_monkey(monkeys, &b);
            let Monkey::Value(a) = &*monkeys.get(a).unwrap().borrow() else { unreachable!() };
            let Monkey::Value(b) = &*monkeys.get(b).unwrap().borrow() else { unreachable!() };

            *current = Monkey::Value(a - b);
        }
        Monkey::Multiply(a, b) => {
            eval_monkey(monkeys, &a);
            eval_monkey(monkeys, &b);
            let Monkey::Value(a) = &*monkeys.get(a).unwrap().borrow() else { unreachable!() };
            let Monkey::Value(b) = &*monkeys.get(b).unwrap().borrow() else { unreachable!() };

            *current = Monkey::Value(a * b);
        }
        Monkey::Divide(a, b) => {
            eval_monkey(monkeys, &a);
            eval_monkey(monkeys, &b);
            let Monkey::Value(a) = &*monkeys.get(a).unwrap().borrow() else { unreachable!() };
            let Monkey::Value(b) = &*monkeys.get(b).unwrap().borrow() else { unreachable!() };

            *current = Monkey::Value(a / b);
        }
    };
}
