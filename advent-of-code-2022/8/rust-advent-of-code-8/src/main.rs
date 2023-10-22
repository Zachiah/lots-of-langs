use std::fs::File;
use std::io::{BufRead, BufReader};

fn all_less_than(values: &[u32], value: u32) -> bool {
    values.iter().all(|item| *item < value)
}

fn main() {
    let file = File::open("../data.txt").expect("Failed to open file");
    let grid: Vec<Vec<u32>> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid number in file"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let pt1_res = grid
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter(|(column_index, value)| {
                    let column = grid
                        .iter()
                        .map(|row| row[*column_index])
                        .collect::<Vec<_>>();
                    all_less_than(&row[0..*column_index], **value)
                        || all_less_than(&row[column_index + 1..], **value)
                        || all_less_than(&column[0..row_index], **value)
                        || all_less_than(&column[row_index + 1..], **value)
                })
                .collect::<Vec<_>>()
        })
        .count();

    let pt2_res = grid
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(column_index, value)| {
                    let column = grid.iter().map(|row| row[column_index]).collect::<Vec<_>>();

                    let viewpoints: [Vec<_>; 4] = [
                        row[0..column_index].iter().rev().collect(),
                        row[column_index + 1..].iter().collect(),
                        column[0..row_index].iter().rev().collect(),
                        column[row_index + 1..].iter().collect(),
                    ];

                    viewpoints
                        .iter()
                        .map(|v| {
                            if v.len() == 0 {
                                0
                            } else {
                                v[1..]
                                    .iter()
                                    .enumerate()
                                    .take_while(|(idx, _item)| v[*idx] < value)
                                    .count()
                                    + 1
                            }
                        })
                        .reduce(|a, b| a * b)
                })
                .max()
                .expect("There should be more than zero trees in this row")
        })
        .max()
        .expect("There should be more than zero trees");

    println!("Pt1 Result: {}", pt1_res);
    println!("Pt2 Result: {}", pt2_res);
}
