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
            Monkey::Equation(Equation::Value(data[0].parse().ok()))
        } else if name == "root" {
            Monkey::Equals(data[0].to_owned(), data[2].to_owned())
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
    *monkeys["humn"].borrow_mut() = Monkey::Equation(Equation::Value(None));

    let evaluated = monkeys.clone();
    eval_monkey(&monkeys, "root");

    let root = monkeys["root"].borrow().clone();
    let humn = match root {
        Monkey::Equation(Equation::Equals(a, b)) => {
            let (mut value, mut eqn) = match (*a, *b) {
                (a, Equation::Value(b)) => (b.unwrap(), a),
                (Equation::Value(a), b) => (a.unwrap(), b),
                _ => unreachable!(),
            };

            let val = loop {
                match eqn {
                    Equation::Value(_) => {
                        break value;
                    }
                    Equation::Add(ref a, ref b) => {
                        let (v, e) = match (*a.clone(), *b.clone()) {
                            (a, Equation::Value(b)) => (b.unwrap(), a),
                            (Equation::Value(a), b) => (a.unwrap(), b),
                            _ => unreachable!(),
                        };

                        value -= v;
                        eqn = e;
                    }
                    Equation::Subtract(a, b) => match (*a.clone(), *b.clone()) {
                        (a, Equation::Value(b)) => {
                            let (v, e) = (b.unwrap(), a);
                            value += v;
                            eqn = e;
                        }
                        (Equation::Value(a), b) => {
                            let (v, e) = (a.unwrap(), b);
                            value = -value + v;
                            eqn = e;
                        }
                        _ => unreachable!(),
                    },

                    Equation::Multiply(a, b) => {
                        let (v, e) = match (*a.clone(), *b.clone()) {
                            (a, Equation::Value(b)) => (b.unwrap(), a),
                            (Equation::Value(a), b) => (a.unwrap(), b),
                            _ => unreachable!(),
                        };

                        value /= v;
                        eqn = e;
                    }

                    Equation::Divide(a, b) => match (*a.clone(), *b.clone()) {
                        (a, Equation::Value(b)) => {
                            let (v, e) = (b.unwrap(), a);
                            value *= v;
                            eqn = e;
                        }
                        (Equation::Value(a), b) => {
                            let (v, e) = (a.unwrap(), b);
                            value = v / value;
                            eqn = e;
                        }
                        _ => unreachable!(),
                    },

                    _ => unreachable!(),
                }
            };
            val
        }

        _ => unreachable!(),
    };

    println!("{humn}");
    *evaluated["humn"].borrow_mut() = Monkey::Equation(Equation::Value(Some(humn)));
    eval_monkey(&evaluated, "root");

    dbg!(&evaluated["root"]);
}

#[derive(Debug, Clone, PartialEq)]
enum Monkey {
    Equation(Equation),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
    Equals(String, String),
}

#[derive(PartialEq, Clone)]
enum Equation {
    Value(Option<i64>),
    Add(Box<Equation>, Box<Equation>),
    Subtract(Box<Equation>, Box<Equation>),
    Multiply(Box<Equation>, Box<Equation>),
    Divide(Box<Equation>, Box<Equation>),
    Equals(Box<Equation>, Box<Equation>),
}

impl std::fmt::Debug for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Equation::Value(v) => match v {
                Some(v) => f.write_fmt(format_args!("{:?}", v)),
                None => f.write_str("HUMN"),
            },
            Equation::Add(a, b) => f.write_fmt(format_args!("({:?} + {:?})", a, b)),
            Equation::Subtract(a, b) => f.write_fmt(format_args!("({:?} - {:?})", a, b)),
            Equation::Multiply(a, b) => f.write_fmt(format_args!("({:?} * {:?})", a, b)),
            Equation::Divide(a, b) => f.write_fmt(format_args!("({:?} / {:?})", a, b)),
            Equation::Equals(a, b) => f.write_fmt(format_args!("({:?} = {:?})", a, b)),
        }
    }
}

fn eval_monkey(monkeys: &HashMap<String, RefCell<Monkey>>, current: &str) {
    let current = &mut *monkeys.get(current).unwrap().borrow_mut();

    match current {
        Monkey::Equation(_) => {
            return;
        }
        Monkey::Add(a, b) => {
            eval_monkey(monkeys, &a);
            eval_monkey(monkeys, &b);
            let Monkey::Equation(a) = &*monkeys.get(a).unwrap().borrow() else { unreachable!() };
            let Monkey::Equation(b) = &*monkeys.get(b).unwrap().borrow() else { unreachable!() };

            match (a, b) {
                (Equation::Value(Some(a)), Equation::Value(Some(b))) => {
                    *current = Monkey::Equation(Equation::Value(Some(*a + *b)));
                }
                _ => {
                    *current =
                        Monkey::Equation(Equation::Add(Box::new(a.clone()), Box::new(b.clone())));
                }
            }
        }
        Monkey::Subtract(a, b) => {
            eval_monkey(monkeys, &a);
            eval_monkey(monkeys, &b);
            let Monkey::Equation(a) = &*monkeys.get(a).unwrap().borrow() else { unreachable!() };
            let Monkey::Equation(b) = &*monkeys.get(b).unwrap().borrow() else { unreachable!() };

            match (a, b) {
                (Equation::Value(Some(a)), Equation::Value(Some(b))) => {
                    *current = Monkey::Equation(Equation::Value(Some(*a - *b)));
                }
                _ => {
                    *current = Monkey::Equation(Equation::Subtract(
                        Box::new(a.clone()),
                        Box::new(b.clone()),
                    ));
                }
            }
        }
        Monkey::Multiply(a, b) => {
            eval_monkey(monkeys, &a);
            eval_monkey(monkeys, &b);
            let Monkey::Equation(a) = &*monkeys.get(a).unwrap().borrow() else { unreachable!() };
            let Monkey::Equation(b) = &*monkeys.get(b).unwrap().borrow() else { unreachable!() };

            match (a, b) {
                (Equation::Value(Some(a)), Equation::Value(Some(b))) => {
                    *current = Monkey::Equation(Equation::Value(Some(*a * *b)));
                }
                _ => {
                    *current = Monkey::Equation(Equation::Multiply(
                        Box::new(a.clone()),
                        Box::new(b.clone()),
                    ));
                }
            }
        }
        Monkey::Divide(a, b) => {
            eval_monkey(monkeys, &a);
            eval_monkey(monkeys, &b);
            let Monkey::Equation(a) = &*monkeys.get(a).unwrap().borrow() else { unreachable!() };
            let Monkey::Equation(b) = &*monkeys.get(b).unwrap().borrow() else { unreachable!() };

            match (a, b) {
                (Equation::Value(Some(a)), Equation::Value(Some(b))) => {
                    *current = Monkey::Equation(Equation::Value(Some(*a / *b)));
                }
                _ => {
                    *current = Monkey::Equation(Equation::Divide(
                        Box::new(a.clone()),
                        Box::new(b.clone()),
                    ));
                }
            }
        }
        Monkey::Equals(a, b) => {
            eval_monkey(monkeys, &a);
            eval_monkey(monkeys, &b);
            let Monkey::Equation(a) = &*monkeys.get(a).unwrap().borrow() else { unreachable!() };
            let Monkey::Equation(b) = &*monkeys.get(b).unwrap().borrow() else { unreachable!() };

            *current = Monkey::Equation(Equation::Equals(Box::new(a.clone()), Box::new(b.clone())));
        }
    };
}
