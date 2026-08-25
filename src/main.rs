use std::io::{Write, stdin};

enum Actions {
    Exit,
    Create,
}

enum LoopActions {
    Continue,
    Break,
}

impl Actions {
    pub fn from_input(text: &str) -> Option<Actions> {
        match text {
            ":q" => Some(Actions::Exit),
            ":c" => Some(Actions::Create),
            other => {
                println!("{other} is not a valid action!");
                None
            }
        }
    }
}

// Fn(String) -> LoopAction

fn input_loop<T>(funct: T)
where
    T: Fn(String) -> LoopActions,
{
    loop {
        let stdin = stdin();
        let mut buffer = String::new();

        stdin.read_line(&mut buffer).expect("couldn't read line");
        match funct(buffer) {
            LoopActions::Break => {
                break;
            }
            LoopActions::Continue => {
                continue;
            }
        }
    }
}

fn main() {
    loop {
        let stdin = stdin();
        let mut buffer = String::new();

        stdin.read_line(&mut buffer).expect("couldn't read line");
        println!("You typed {buffer}");
        match Actions::from_input(buffer.trim()) {
            Some(Actions::Exit) => {
                break;
            }
            Some(Actions::Create) => loop {
                let mut name = String::new();
                stdin.read_line(&mut name).expect("couldnt read file name");

                let file_res = std::fs::File::create_new(&name);
                match file_res {
                    Err(er) => {
                        println!("{er}");
                    }
                    Ok(mut file) => {
                        println!("created!");
                        break;
                    }
                }
            },
            None => {
                continue;
            }
        }
    }
}
