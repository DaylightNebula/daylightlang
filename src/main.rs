pub mod lexical;
pub mod enums;
pub mod ll_compiler;

fn main() {
    // read input and compile it into statements
    let input = std::fs::read_to_string("./test/helloWorld.day");
    let tree = lexical::build_tree(input.unwrap().as_str());

    // generate result
    let result = ll_compiler::compile(tree);
    // println!("Compiled Result:");
    // for string in result {
    //     println!("{}", string);
    // }

    std::fs::write("./test/compiled.ll", result.join("\n"));
}
