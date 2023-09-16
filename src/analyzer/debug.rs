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

    output.push("\nFUNCTIONS: ".to_string());
    for context in &analysis.functions {
        output.push(format!(
            "fun {}({}): {} {{}}", 
            context.name, 
            context.input_args.iter().map(|a| format!("{}: {}", a.name, a.type_wrapper.to_str())).collect::<Vec<String>>().join(", "), 
            context.ret_type.to_str()
        ));
    }

    output.join("\n")
}