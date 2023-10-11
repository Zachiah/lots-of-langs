use std::io::BufRead;
use std::{fs, io};

#[derive(Debug)]
struct State {
    highest: [i32; 3],
    acc: i32,
}
impl State {
    fn add(mut self, amount: i32) -> Self {
        self.acc += amount;
        self
    }

    fn apply(mut self) -> Self {
        match self.highest.iter().position(|&val| self.acc > val) {
            None => {}
            Some(index) => {
                for after_index in (index + 1..self.highest.len()).rev() {
                    self.highest[after_index] = self.highest[after_index - 1];
                }

                self.highest[index] = self.acc;
            }
        }

        self.acc = 0;
        self
    }

    fn result(self) -> i32 {
        self.highest.iter().sum()
    }
}

fn main() {
    let file = fs::File::open("../data.txt").expect("data.txt should exist");
    let res = io::BufReader::new(file)
        .lines()
        .fold(
            State {
                highest: [0; 3],
                acc: 0,
            },
            |state, curr| {
                let line = curr.expect("failed to read line from data.txt");

                match line.as_str() {
                    "" => state.apply(),
                    line => state.add(line.parse::<i32>().expect("Non number found in the file")),
                }
            },
        )
        .result();
    println!("{}", res);
}
