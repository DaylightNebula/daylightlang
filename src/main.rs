use std::{env, fs::DirEntry};

pub mod analyzer;
pub mod ir_compiler;
pub mod tokenizer;

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
    println!("Processing {} ...", entry.path().to_str().unwrap());
    let formatted_path = format!(
        "{}/{}", 
        entry.path().as_path().to_str().unwrap(), 
        entry.file_name().to_str().unwrap()
    );

    // read file content
    let content = std::fs::read_to_string(
        format!("{}.day", formatted_path)
    ).unwrap();
    let content = content.as_str();
    
    // run tokenize step
    let lines = tokenizer::breakup_text(content, false);
    let lines_str = tokenizer::debug::convert_lines_to_string(&lines, 0);
    let _ = std::fs::write(format!("{}.tokens.txt", formatted_path), lines_str);

    // run analysis
    let analysis = analyzer::analyze_root(lines);
    let analysis_str = analyzer::debug::convert_analysis_to_string(&analysis);
    let _ = std::fs::write(format!("{}.analysis.txt", formatted_path), analysis_str);
    
    // do final compile
    let compiled = ir_compiler::compile_analysis(analysis).join("\n");
    println!("Final code: \n{}", compiled);
    let _ = std::fs::write(format!("{}.ll", formatted_path), compiled);
}