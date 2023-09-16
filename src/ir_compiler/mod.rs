use crate::analyzer::{Analysis, arguments::LLVMArgument};

/***************
 * The goal of this module is to compile an analysis from
 * the analyzer into LLVM IR.
 ***************/

pub fn compile_analysis(analysis: Analysis) -> Vec<String> {
    // create output vector
    let possible_size = analysis.constants.len();
    let mut output = Vec::with_capacity(possible_size);

    // add constants
    for (c_name, c_value) in analysis.constants {
        output.push(format!("@.{} = private constant {} {}", c_name, c_value.to_llvm_type_str(), c_value.to_llvm_value()));
    }

    for (e_name, e, t) in analysis.externs {
        output.push(format!("declare {} @{}({})", t.to_str(), e_name, LLVMArgument::list_to_llvm(&e)))
    }

    // pass back output
    return output;
}
