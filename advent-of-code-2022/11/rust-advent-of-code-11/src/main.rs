use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: Test,
    inspected_count: i64,
}

#[derive(Clone, Debug)]
struct Operation {
    a: Operand,
    b: Operand,
    operator: Operator,
}

impl Operation {
    fn exec(&self, old: i64) -> i64 {
        let a = self.a.get(old);
        let b = self.b.get(old);

        self.operator.run(a, b)
    }
}

#[derive(Clone, Debug)]
enum Operator {
    Plus,
    Times,
}

impl Operator {
    fn run(&self, a: i64, b: i64) -> i64 {
        match self {
            Self::Plus => a + b,
            Self::Times => a * b,
        }
    }
}

#[derive(Clone, Debug)]
enum Operand {
    Number(i64),
    Old,
}

impl Operand {
    fn get(&self, old: i64) -> i64 {
        match self {
            Self::Number(i) => *i,
            Self::Old => old,
        }
    }
}

#[derive(Clone, Debug)]
struct Test {
    divisible_by: i64,
    if_throw_to: usize,
    else_throw_to: usize,
}
impl FromStr for Operand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Self::Old),
            n => match n.parse() {
                Ok(num) => Ok(Self::Number(num)),
                Err(_) => Err(()),
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Monkeys(Vec<Monkey>);

impl Monkeys {
    fn remove_max(&mut self) -> Monkey {
        self.0
            .remove(self.0.iter().enumerate().fold(0, |max_idx, (idx, m)| {
                if m.inspected_count > self.0[max_idx].inspected_count {
                    idx
                } else {
                    max_idx
                }
            }))
    }

    fn calculate_result(&mut self) -> i64 {
        self.remove_max().inspected_count * self.remove_max().inspected_count
    }

    fn apply_item(&mut self, item: i64, test: Test) {
        if item % test.divisible_by == 0 {
            self.0[test.if_throw_to].items.push(item);
        } else {
            self.0[test.else_throw_to].items.push(item);
        }
    }

    fn process_monkey(&mut self, idx: usize, get_new_item: impl Fn(i64) -> i64) {
        let num_monkeys = self.0.len();
        loop {
            let monkey = &mut self.0[idx % num_monkeys];
            if monkey.items.len() == 0 {
                break;
            };
            monkey.inspected_count += 1;
            let item = monkey.items.remove(0);

            let monkey = monkey.clone();
            let new_item = get_new_item(monkey.operation.exec(item));
            self.apply_item(new_item, monkey.test);
        }
    }
}

impl FromStr for Operator {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Plus),
            "*" => Ok(Self::Times),
            _ => Err(()),
        }
    }
}

fn main() {
    let file = File::open("../data.txt").expect("Failed to open file");
    let mut lines = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error while reading line"));

    let mut monkeys: Monkeys = Monkeys(vec![]);
    while let Some(_) = lines.next() {
        let items = lines
            .next()
            .expect("Should contain starting items")
            .trim_start()
            .trim_start_matches("Starting items: ")
            .split(", ")
            .map(|n| {
                n.parse::<i64>()
                    .expect("Failed to read number from starting item")
            })
            .collect::<Vec<_>>();

        let operation = {
            let split = lines
                .next()
                .expect("Should contain operation")
                .split(' ')
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();

            Operation {
                a: split[5].parse().ok().expect("Should be a first operand"),
                operator: split[6].parse().ok().expect("Should be an operator"),
                b: split[7].parse().ok().expect("Should be a second operand"),
            }
        };

        let test = {
            let divisible_by: i64 = lines
                .next()
                .expect("There should be a test")
                .trim_start()
                .trim_start_matches("Test: divisible by ")
                .parse()
                .expect("Divisible by should be a number");

            let if_throw_to: usize = lines
                .next()
                .expect("There should be a if true line")
                .trim_start()
                .trim_start_matches("If true: throw to monkey ")
                .parse()
                .expect("throw to should be a usize");

            let else_throw_to: usize = lines
                .next()
                .expect("There should be a if true line")
                .trim_start()
                .trim_start_matches("If false: throw to monkey ")
                .parse()
                .expect("throw to should be a usize");

            Test {
                divisible_by,
                if_throw_to,
                else_throw_to,
            }
        };

        lines.next();

        monkeys.0.push(Monkey {
            items,
            operation,
            test,
            inspected_count: 0,
        });
    }

    let pt1_result = {
        let mut monkeys = monkeys.clone();
        for idx in 0..20 * monkeys.0.len() {
            monkeys.process_monkey(idx, |v| v / 3);
        }
        monkeys.calculate_result()
    };

    let pt2_result = {
        let global_modulo = monkeys
            .0
            .iter()
            .map(|m| m.test.divisible_by)
            .fold(1, |a, b| a * b);
        for idx in 0..10_000 * monkeys.0.len() {
            monkeys.process_monkey(idx, |v| v % global_modulo);
        }
        monkeys.calculate_result()
    };

    println!("Pt1 Res: {}", pt1_result);
    println!("Pt2 Res: {}", pt2_result);
}
