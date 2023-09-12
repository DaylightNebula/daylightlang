use std::{env, fs::DirEntry};

pub mod lexical;

fn main() {
    // if basic examples option enabled, run them
    if env::args().collect::<Vec<String>>().contains(&"basic-examples".to_string()) {
        run_basic_examples();
    }
}

fn run_basic_examples() {
    // loop through all basic examples and run them
    for path in std::fs::read_dir("./basic_examples").unwrap() {
        if path.is_ok() {
            run_basic_example(path.unwrap());
        }
    }
}

fn run_basic_example(entry: DirEntry) {
    // read file content
    let content = std::fs::read_to_string(
        format!(
            "{}/{}.day", 
            entry.path().as_path().to_str().unwrap(), 
            entry.file_name().to_str().unwrap()
        )
    ).unwrap();
    
}