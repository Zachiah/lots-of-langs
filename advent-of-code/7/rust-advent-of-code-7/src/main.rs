use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::rc::Rc;

struct State {
    fs: Rc<RefCell<StateFsFolder>>,
    current_dir: Vec<Rc<RefCell<StateFsFolder>>>, 
    in_ls: bool,
}
    
impl State {
    fn new() -> Self {
        let fs = Rc::new(RefCell::new(StateFsFolder {name: "/".to_string(), contents: vec!()}));
        State {
            fs: fs.clone(),
            current_dir: vec!(fs.clone()),
            in_ls: false
        }
    }

    fn insert_command(mut self, command: Command) -> Self {
        match command {
            Command::Cd(name) => match name.as_str() {
                ".." => {self.current_dir.pop().expect("Can't go up from root");},
                d => {

                    let new_single_current_dir = { 
                        let mut single_current_dir = self.current_dir.last().expect("Should always be a last").borrow_mut();
                    
                        single_current_dir
                            .contents
                            .iter()
                            .find(|f| match f {
                                StateFs::File(file) => file.name == d, 
                                StateFs::Folder(dir) => dir.borrow().name == d,
                            })
                            .map(|f| match f {
                                StateFs::File(_) => panic!("Can't cd into file"),
                                StateFs::Folder(dir) => dir.clone(),
                            })
                            .unwrap_or_else(|| {
                                let new_dir = Rc::new(RefCell::new(StateFsFolder { name: d.to_string(), contents: vec!()}));
                                single_current_dir.contents.push(StateFs::Folder(new_dir.clone()));
                                new_dir.clone()
                            })
                    };

                    self.current_dir.push(new_single_current_dir.clone());
                }
            },
            Command::Ls => self.in_ls = true
        }
        self
    }

    fn insert_dir(self, dirname: &str) -> Self {
        self
            .current_dir
            .last()
            .expect("Should always be a last")
            .borrow_mut()
            .contents
            .push(
                StateFs::Folder(
                    Rc::new(RefCell::new(StateFsFolder {
                        name: dirname.to_string(), 
                        contents: vec!(),
                    }))
                )
            );
        self
    }

    fn insert_file(self, file: StateFsFile) -> Self {
        self
            .current_dir
            .last()
            .expect("Should always be a last")
            .borrow_mut()
            .contents
            .push(
                StateFs::File(file)
            );
        self
    }
}

struct StateFsFile {size: i32, name: String}
struct StateFsFolder {name: String, contents: Vec<StateFs>}

impl StateFsFolder {
    fn calculate_pt1(&self) -> i32 {
        self.walk_dir_sizes().filter(|&s| s <= 100_000).sum()
    }

    fn calculate_pt2(&self) -> i32 {
        let free_space = 70_000_000 - self.size();
        let to_remove = 30_000_000 - free_space;
        
        self.walk_dir_sizes().filter(|&s| s >= to_remove).min().expect("Unable to find any directories large enough")
    }

    fn walk_dir_sizes(&self) -> impl Iterator<Item = i32> + '_ {
        iter::once(self.size()).chain(self.contents.iter().filter_map(|f| match f {
            StateFs::File(_) => None,
            StateFs::Folder(f) => Some(f)
        }).flat_map(|f| f.borrow().walk_dir_sizes().collect::<Vec<_>>()))
    }

    fn size(&self) -> i32 {
        self
            .contents
            .iter()
            .map(|f| match f {
                StateFs::File(file) => file.size,
                StateFs::Folder(folder) => folder.borrow().size(),
            })
            .sum()
    }
}

enum StateFs {
    File(StateFsFile),
    Folder(Rc<RefCell<StateFsFolder>>),
}

enum Command { Cd(String), Ls }

fn main() {
    let file = File::open("../data.txt").expect("Failed to read file");

    let res = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error while reading file"))
        .map(|l| l.split(' ').map(|s| s.to_string()).collect::<Vec<_>>())
        .fold(State::new(), |state, line| match line[0].as_str() {
                "$" => state.insert_command(match line[1].as_str() {
                    "cd" => Command::Cd(line[2].to_string()),
                    "ls" => Command::Ls,
                    c => panic!("Invalid command: {}", c),
                }),
                "dir" => state.insert_dir(&line[1]),
                potential_number => match potential_number.parse::<i32>() {
                    Ok(size) => state.insert_file(StateFsFile { size, name: line[1].to_string() }),
                    Err(_) => panic!("Invalid line"),
                },
        });

    println!("Pt1 res: {}", res.fs.borrow().calculate_pt1());
    println!("Pt2 res: {}", res.fs.borrow().calculate_pt2());
}
