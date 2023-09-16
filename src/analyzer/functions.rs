use super::{arguments::LLVMArgument, types::LLVMTypeWrapper};

#[derive(Debug, Clone)]
pub struct FunctionContext {
    pub name: String,
    pub input_args: Vec<LLVMArgument>,
    pub variables: Vec<(LLVMArgument, usize)>,
    pub var_counter: usize,
    pub ret_type: LLVMTypeWrapper
}

impl FunctionContext {
    pub fn new(name: String, args: Vec<LLVMArgument>, ret_type: LLVMTypeWrapper) -> Self {
        Self {
            name, input_args: args.clone(),
            variables: args.iter().enumerate().map(|(b, a)| (a.clone(), b)).collect::<Vec<(LLVMArgument, usize)>>(),
            var_counter: args.len(), ret_type
        }
    }
}