pub mod lexical;
pub mod enums;

pub struct CompileInfo {

}

fn main() {
    let input = std::fs::read_to_string("./test/helloWorld.day");
    let tree = lexical::build_tree(input.unwrap().as_str());
    println!("Tree:");
    for segment in tree {
        println!("Segment: {:?}", segment)
    }
}
