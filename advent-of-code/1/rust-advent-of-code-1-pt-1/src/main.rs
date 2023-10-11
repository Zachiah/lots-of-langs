use std::{fs, io};
use std::io::BufRead;

fn main() {
    let file = fs::File::open("../data.txt").expect("data.txt should exist"); 
    let res: i32 = io::BufReader::new(file).lines().fold((0,0), |acc,curr| {
        let line = curr.expect("failed to read line from data.txt");

        println!("{}, {} - {}", acc.0, acc.1, line);
        match line.as_str() {
            "" => ( 0, (acc.0).max(acc.1) ),
            line => (acc.0 + line.parse::<i32>().expect("Non number found in the file"), acc.1)
        } 
    }).1;
    println!("{}", res);
}
