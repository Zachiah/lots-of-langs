use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }

    fn add_delta(&self, delta: Delta) -> Self {
        Self {
            x: self.x + delta.x,
            y: self.y + delta.y,
        }
    }

    fn dist(&self, other: &Point) -> Delta {
        Delta {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

struct Delta {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn to_delta(&self) -> Delta {
        match *self {
            Move::Up => Delta { x: 0, y: 1 },
            Move::Down => Delta { x: 0, y: -1 },
            Move::Left => Delta { x: -1, y: 0 },
            Move::Right => Delta { x: 1, y: 0 },
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    knots: Vec<Point>,
    tail_position_history: HashSet<Point>,
}

impl State {
    fn new(num_knots: usize) -> Self {
        let mut tail_position_history = HashSet::new();
        tail_position_history.insert(Point::origin());
        State {
            knots: iter::repeat(Point::origin()).take(num_knots).collect(),
            tail_position_history,
        }
    }

    fn advance_tail(mut self) -> Self {
        self.knots = iter::once(self.knots[0].clone())
            .chain(self.knots.windows(2).map(|items| {
                let [prev, curr] = items else {
                    panic!("Should be 2")
                };
                let delta = prev.dist(curr);

                if delta.x.abs() <= 1 && delta.y.abs() <= 1 {
                    curr.clone()
                } else {
                    curr.add_delta(Delta {
                        x: if delta.x == 0 {
                            0
                        } else {
                            delta.x / delta.x.abs()
                        },
                        y: if delta.y == 0 {
                            0
                        } else {
                            delta.y / delta.y.abs()
                        },
                    })
                }
            }))
            .collect();

        self.tail_position_history.insert(
            self.knots
                .last()
                .expect("There will be more than 0 knots")
                .clone(),
        );

        return self;
    }
    fn add_move(mut self, m: &Move) -> Self {
        self.knots[0] = self.knots[0].add_delta(m.to_delta());
        return self;
    }
}

fn main() {
    let file = File::open("../data.txt").expect("Failed to read file");

    let (pt1_state, pt2_state) = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .flat_map(|line| {
            let split = line.split(" ").collect::<Vec<_>>();
            let amount = split[1].parse::<usize>().expect("Failed to parse number");
            match split[0] {
                "D" => iter::repeat(Move::Down).take(amount),
                "U" => iter::repeat(Move::Up).take(amount),
                "L" => iter::repeat(Move::Left).take(amount),
                "R" => iter::repeat(Move::Right).take(amount),
                m => panic!("Failed to parse move {}", m),
            }
        })
        .fold((State::new(2), State::new(10)), |(state1, state2), m| {
            (
                state1.add_move(&m).advance_tail(),
                state2.add_move(&m).advance_tail(),
            )
        });

    println!("Pt1 Res: {}", pt1_state.tail_position_history.len());
    println!("Pt2 Res: {}", pt2_state.tail_position_history.len());
}
