use std::fs::File;
use std::io::{BufRead, BufReader};

struct CPU {
    instructions: Vec<Instruction>,
    cursor: usize,
    x: i32,
    instruction: Instruction,
    cycle_count: i32,
}

impl CPU {
    fn new(instructions: Vec<Instruction>) -> CPU {
        CPU {
            instruction: instructions[0].clone(),
            instructions,
            cursor: 0,
            x: 1,
            cycle_count: 1,
        }
    }

    fn cycle(&mut self) -> Option<()> {
        self.cycle_count += 1;
        match self.instruction {
            Instruction::Noop => self.load_instruction()?,
            Instruction::AddxBegin(amount) => {
                self.instruction = Instruction::AddxEnd(amount);
            }
            Instruction::AddxEnd(amount) => {
                self.x += amount;
                self.load_instruction()?;
            }
        }

        Some(())
    }

    fn load_instruction(&mut self) -> Option<()> {
        if self.cursor + 1 >= self.instructions.len() {
            None
        } else {
            self.cursor += 1;
            self.instruction = self.instructions[self.cursor].clone();
            Some(())
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Noop,
    AddxBegin(i32),
    AddxEnd(i32),
}

struct CRT {
    pos: usize,
    screen: [[PixelState; 40]; 6],
}

impl CRT {
    fn new() -> Self {
        Self {
            pos: 0,
            screen: [[PixelState::Empty; 40]; 6],
        }
    }

    fn set(&mut self, sprite_pos: i32) {
        let pos: i32 = self.pos.try_into().unwrap();
        let x_pos = pos % 40;
        self.screen[self.pos / 40][self.pos % 40] =
            if sprite_pos == x_pos || sprite_pos - 1 == x_pos || sprite_pos + 1 == x_pos {
                PixelState::Filled
            } else {
                PixelState::Empty
            };

        self.pos += 1;
    }
}

struct State {
    cpu: CPU,
    crt: CRT,
}

impl State {
    fn new(instructions: Vec<Instruction>) -> Self {
        State {
            cpu: CPU::new(instructions),
            crt: CRT::new(),
        }
    }

    fn cycle(&mut self) -> Option<()> {
        self.crt.set(self.cpu.x);
        self.cpu.cycle()?;

        Some(())
    }
}

#[derive(Debug, Clone, Copy)]
enum PixelState {
    Filled,
    Empty,
}

impl PixelState {
    fn print(&self) -> char {
        return match self {
            PixelState::Empty => ' ',
            PixelState::Filled => '#',
        };
    }
}
fn main() {
    let file = File::open("../data.txt").expect("Failed to read input file");
    let lines = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error while reading input file"))
        .map(|line| {
            let split = line.split(" ").collect::<Vec<_>>();
            match split[0] {
                "noop" => Instruction::Noop,
                "addx" => Instruction::AddxBegin(split[1].parse().expect("Failed to read integer")),
                invalid_word => panic!("Invalid word: {}", invalid_word),
            }
        })
        .collect::<Vec<_>>();

    let mut state = State::new(lines);

    let mut total = 0;
    while state.cycle().is_some() {
        if [20, 60, 100, 140, 180, 220].contains(&state.cpu.cycle_count) {
            total += state.cpu.cycle_count * state.cpu.x;
        }
    }

    println!("Pt1 Res: {}", total);

    println!(
        "\n\nPt2 Res:\n\n{}\n",
        state
            .crt
            .screen
            .map(|row| row.map(|p| p.print()).iter().collect::<String>())
            .join("\n")
    )
}
