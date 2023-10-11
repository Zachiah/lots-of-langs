use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, PartialEq, Eq)]
enum HandPos {
    Rock,
    Paper,
    Scissors,
}

impl HandPos {
    fn loses_to(&self) -> Self {
        match self {
            HandPos::Rock => HandPos::Paper,
            HandPos::Paper => HandPos::Scissors,
            HandPos::Scissors => HandPos::Rock,
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            HandPos::Rock => HandPos::Scissors,
            HandPos::Paper => HandPos::Rock,
            HandPos::Scissors => HandPos::Paper,
        }
    }

    fn draws_against(&self) -> Self {
        self.clone()
    }
}

enum GameResult {
    Win,
    Loss,
    Draw,
}

struct Game(HandPos, HandPos);

impl Game {
    fn play(&self) -> GameResult {
        match self {
            Game(a, b) if a.wins_against() == *b => GameResult::Win,
            Game(a, b) if a.loses_to() == *b => GameResult::Loss,
            Game(a, b) if a.draws_against() == *b => GameResult::Draw,
            _ => panic!("This should never be reached, there is a logic error"),
        }
    }

    fn from_line_pt_1(line: &str) -> Game {
        let mut chars = line.split(" ");

        let first = match chars.next().expect("Line missing first part") {
            "A" => HandPos::Rock,
            "B" => HandPos::Paper,
            "C" => HandPos::Scissors,
            _ => panic!("Invalid first part"),
        };

        let second = match chars.next().expect("Line missing second part") {
            "X" => HandPos::Rock,
            "Y" => HandPos::Paper,
            "Z" => HandPos::Scissors,
            _ => panic!("Invalid second part"),
        };

        Game(second, first)
    }

    fn from_line_pt_2(line: &str) -> Game {
        let mut chars = line.split(" ");

        let first = match chars.next().expect("Line missing first part") {
            "A" => HandPos::Rock,
            "B" => HandPos::Paper,
            "C" => HandPos::Scissors,
            _ => panic!("Invalid first part"),
        };

        let second = match chars.next().expect("Line missing second part") {
            "X" => first.wins_against(),
            "Y" => first.draws_against(),
            "Z" => first.loses_to(),
            _ => panic!("Invalid second part"),
        };

        Game(second, first)
    }

    fn score(&self) -> i32 {
        let pos_score = match self.0 {
            HandPos::Rock => 1,
            HandPos::Paper => 2,
            HandPos::Scissors => 3,
        };

        let outcome_score = match self.play() {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Loss => 0,
        };

        pos_score + outcome_score
    }
}

fn main() {
    let file = File::open("../data.txt").expect("Data file doesn't exist");
    let res: (i32, i32) = BufReader::new(file)
        .lines()
        .map(|line_res| {
            let line = line_res.expect("Failed to read line");

            let game_pt_1 = Game::from_line_pt_1(&line);
            let game_pt_2 = Game::from_line_pt_2(&line);

            (game_pt_1.score(), game_pt_2.score())
        })
        .fold((0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2));

    println!("Part1 Answer: {}", res.0);
    println!("Part2 Answer: {}", res.1);
}
