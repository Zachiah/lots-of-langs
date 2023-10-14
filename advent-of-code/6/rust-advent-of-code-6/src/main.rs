use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_marker(input: &str, len: usize) -> usize {
    input.as_bytes()
        .windows(len)
        .map(|b| String::from_utf8_lossy(b))
        .position(|s| {
            s
                  .chars()
                  .enumerate()
                  .all(|(index,c)| !s[index+1..].contains(c))
        })
        .expect("Failed to find index") + len
}

fn main() {
    let file = File::open("../data.txt").expect("Unable to read file");
    let input = BufReader::new(file)
        .lines()
        .next()
        .expect("Failed to read line")
        .expect("Failed to read line");

    let res_pt1 = find_marker(&input, 4);
    let res_pt2 = find_marker(&input, 14);

    println!("Pt1 res: {}", res_pt1);
    println!("Pt2 res: {}", res_pt2);

}
