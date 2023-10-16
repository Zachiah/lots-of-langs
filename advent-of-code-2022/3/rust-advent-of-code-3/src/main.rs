use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn get_priority(c: char) -> u32 {
    match c {
        c if c >= 'a' && c <= 'z' => (c as u32) - ('a' as u32) + 1,
        c if c >= 'A' && c <= 'Z' => (c as u32) - ('A' as u32) + 27,
        c => panic!("Found non letter in data file, {}", c),
    }
}

fn main() {
    let file = File::open("../data.txt").expect("Failed to read data file");

    let rucksacks = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error while reading file"))
        .collect::<Vec<String>>();

    let p1_res: u32 = rucksacks
        .iter()
        .map(|line| (line[0..line.len()/2].to_owned(), line[line.len()/2..].to_owned()))
        .map(|(left,right)| left.chars().find(|c| right.contains(*c)))
        .map(|op| op.expect("Should have a duplicate"))
        .map(get_priority)
        .sum();

    let p2_res: u32 = rucksacks
        .iter()
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let ls: Vec<_> = chunk.collect();

            let first = ls.first().expect("There should always be a first element");
            first.chars().find(|c| ls[1..].iter().all(|line| line.contains(*c))).expect("Couldn't find common char")
        })
        .map(get_priority)
        .sum();

    println!("Part1 Solution: {:?}", p1_res);
    println!("Part2 Solution: {:?}", p2_res);
}

