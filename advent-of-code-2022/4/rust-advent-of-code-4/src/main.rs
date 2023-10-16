use std::ops::RangeInclusive;
use std::fs::File;
use std::io::{BufRead, BufReader};

trait RangeUtils<Idx> {
    fn fully_contains(&self, r: &RangeInclusive<Idx>) -> bool;
    fn partially_contains(&self, r: &RangeInclusive<Idx>) -> bool;
}

impl<Idx> RangeUtils<Idx> for RangeInclusive<Idx> where Idx: PartialOrd {
    fn fully_contains(&self, r: &RangeInclusive<Idx>) -> bool {
        self.start() <= r.start() && self.end() >= r.end()
    }
    fn partially_contains(&self, r: &RangeInclusive<Idx>) -> bool {
        (self.start() <= r.start() && self.end() >= r.start())
            || (self.end() >= r.end() && self.start() <= r.end())
    }
}

fn main() {
    let file = File::open("../data.txt").expect("Failed to open the data file");

    let ranges: Vec<_> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error while reading data file"))
        .map(|l| l
             .split(',')
             .map(|r| r.split('-').map(|n| n.parse::<i32>().expect("Failed to read integer in file")))
             .map(|mut r| r.next().expect("Couldn't read range start")..=r.next().expect("Couldn't read range end"))
             .collect::<Vec<_>>()
        )
        .map(|l| (l[0].clone(), l[1].clone()))
        .collect();
    
    let res_pt1 = ranges
        .iter()
        .filter(|l| l.0.fully_contains(&l.1) || l.1.fully_contains(&l.0))
        .count();

    let res_pt2 = ranges
        .iter()
        .filter(|l| l.0.partially_contains(&l.1) || l.1.partially_contains(&l.0))
        .count();

    println!("Part1 result: {}", res_pt1);
    println!("Part2 result: {}", res_pt2);
}
