use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone)]
enum Item {
    List(Vec<Item>),
    Number(i32),
}

type It<'a> = Peekable<Chars<'a>>;

fn read_number(it: &mut It) -> i32 {
    let mut string_builder = String::new();

    while it
        .peek()
        .expect("There should be another character")
        .is_digit(10)
    {
        string_builder.push(it.next().unwrap());
    }

    string_builder.parse().unwrap()
}

fn read_list(it: &mut It) -> Vec<Item> {
    let mut items: Vec<Item> = vec![];

    it.next();

    loop {
        match it.peek().expect("There should be at least one character") {
            ']' => {
                it.next();
                return items;
            }
            ',' => {
                it.next();
            }
            _ => {
                items.push(read_item(it));
            }
        }
    }
}

fn read_item(it: &mut It) -> Item {
    match it.peek().expect("There should be at least one character") {
        '[' => Item::List(read_list(it)),
        _ => Item::Number(read_number(it)),
    }
}
impl Item {
    fn from_line(line: &str) -> Self {
        let mut it = line.chars().peekable();

        let item = read_item(&mut it);

        it.next()
            .map(|c| panic!("Found character {} after parsing", c));

        item
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
            (Self::List(l1), Self::List(l2)) => l1.iter().cmp(l2),
            (Self::Number(a), Self::List(l2)) => {
                Item::List(vec![Self::Number(*a)]).cmp(&Item::List(l2.clone()))
            }
            (Self::List(l1), Self::Number(b)) => {
                Self::List(l1.clone()).cmp(&Self::List(vec![Self::Number(*b)]))
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Item {}

fn main() {
    let file = File::open("../data.txt").expect("Failed to open file");
    let mut it = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Failed to read line"));

    let mut items: Vec<(Item, Item)> = vec![];
    while let Some(first) = it.next() {
        let second = it.next().expect("There should be a second line");

        items.push((Item::from_line(&first), Item::from_line(&second)));

        it.next();
    }

    println!(
        "Pt1 Result: {}",
        items
            .iter()
            .enumerate()
            .filter(|(_, i)| i.0 < i.1)
            .map(|(idx, _)| idx + 1)
            .sum::<usize>()
    );

    let mut all_items: Vec<_> = items.iter().flat_map(|i| [i.0.clone(), i.1.clone()]).collect();

    let signal1 = Item::List(vec![Item::List(vec![Item::Number(2)])]);
    let signal2 = Item::List(vec![Item::List(vec![Item::Number(6)])]);
    all_items.push(signal1.clone());
    all_items.push(signal2.clone());
    all_items.sort();
    println!(
        "Pt2 Result: {}",
        (all_items
            .iter()
            .position(|i| *i == signal1)
            .expect("Should have signal1")
            + 1)
            * (all_items
                .iter()
                .position(|i| *i == signal2)
                .expect("Should have signal2")
                + 1),
    );
}
