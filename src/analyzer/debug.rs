use super::{Analysis, arguments::LLVMArgument};

pub fn convert_analysis_to_string(analysis: &Analysis) -> String {
    let mut output: Vec<String> = Vec::new();

    output.push("CONSTANTS: ".to_string());
    for (c_name, c) in &analysis.constants {
        output.push(format!("{}: {} = {}", c_name, c.to_llvm_type_str(), c.to_llvm_value()))
    }

    output.push("\nEXTERNS: ".to_string());
    for (e_name, e, t) in &analysis.externs {
        output.push(format!("{}({}): {}", e_name, LLVMArgument::list_to_llvm(&e), t.to_str()));
    }

    output.join("\n")
}