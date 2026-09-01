use std::env;

#[derive(Debug)]
pub struct ArgOptions {
    pub file: Option<String>,
}

pub fn get_options() -> ArgOptions {
    let args: Vec<String> = env::args().collect();
    let file = args.get(1).cloned();
    let result: ArgOptions = ArgOptions { file };
    result
}
