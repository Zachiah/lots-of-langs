use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct State {
    crates: Vec<Vec<char>>,
}

impl State {
    fn get_top_line(&self) -> String {
        self
            .crates
            .iter()
            .map(|c| c[0])
            .collect::<String>()
    }

    fn insert_row(mut self, row: BlockRow) -> Self {
        if row.blocks.len() > self.crates.len() {
            self.crates.extend(vec![vec![];row.blocks.len() - self.crates.len()]);
        }

        for (idx, block) in row.blocks.iter().enumerate() {
            match block {
                None => {},
                Some(c) => self.crates[idx].push(*c),
            }
        }

        self
    }

    fn move_block_pt1(mut self, m: BlockMove) -> Self {
        let mut extended_blocks = {
            let from_row = &mut self.crates[m.from - 1];
            let e = from_row[0..m.amount].iter().rev().map(|c| c.clone()).collect::<Vec<_>>();
            *from_row = from_row[m.amount..].iter().map(|c| *c).collect();
            e
        };

        extended_blocks.extend(self.crates[m.to - 1].clone());
        self.crates[m.to - 1] = extended_blocks;

        self
    }
    

    fn move_block_pt2(mut self, m: BlockMove) -> Self {
        let mut extended_blocks = {
            let from_row = &mut self.crates[m.from - 1];
            let e = from_row[0..m.amount].iter().map(|c| c.clone()).collect::<Vec<_>>();
            *from_row = from_row[m.amount..].iter().map(|c| *c).collect();
            e
        };

        extended_blocks.extend(self.crates[m.to - 1].clone());
        self.crates[m.to - 1] = extended_blocks;

        self
    }

    fn new() -> Self {
        State {
            crates: vec!()
        }
    }
}

struct BlockMove {
    amount: usize,
    from: usize,
    to: usize,
}

impl BlockMove {
    fn from_line(line: &str) -> Self {

        let mut nums = line.split(" ").filter_map(|s| s.parse::<usize>().ok());

        BlockMove {
            amount: nums.next().expect("Failed to read 1st number"),
            from: nums.next().expect("Failed to read 2nd number"),
            to: nums.next().expect("Failed to read 3rd number"),
        }
    }
}

struct BlockRow {
    blocks: Vec<Option<char>>
}

impl BlockRow {
    fn from_line(line: &str) -> Self {
        let num_blocks = (line.len() + 1) / 4;
        
        let result: Vec<Option<char>> = (0..num_blocks).map(|i| {
            let mut s = line[i*4..(i+1)*4-1].chars();

            match s.next().expect("This should always exist") {
                ' ' => None,
                '[' => Some(s.next().expect("This hould always exist")),
                _ => panic!("Invalid input data"),
            }
        }).collect();
        BlockRow {blocks: result}
    }
}

fn main() {
    let file = File::open("../data.txt").expect("Failed to read file");
    

    let res = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error while reading file"))
        .fold((State::new(), State::new()), |(pt1, pt2), line| {
            let mut chars = line.chars();
            match chars.next() {
                None => (pt1, pt2),
                Some(' ') if chars.next() == Some('1') => (pt1,pt2),
                Some(' ') => (pt1.insert_row(BlockRow::from_line(&line)), pt2.insert_row(BlockRow::from_line(&line))),
                Some('m') => (pt1.move_block_pt1(BlockMove::from_line(&line)), pt2.move_block_pt2(BlockMove::from_line(&line))),
                Some('[') => (pt1.insert_row(BlockRow::from_line(&line)), pt2.insert_row(BlockRow::from_line(&line))),
                Some(c) => panic!("Invalid line starting with character: {}", c)
            }
        });

    println!("Pt1 result: {}", res.0.get_top_line());
    println!("Pt2 result: {}", res.1.get_top_line());
}
