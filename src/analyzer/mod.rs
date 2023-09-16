use std::collections::HashMap;

use crate::tokenizer::structs::*;

use self::{constants::LLVMConstant, arguments::LLVMArgument, types::*, functions::FunctionContext};

/********
 * 
 * This module turns the text lines from the tokenizer into final lists of
 * structs, functions, operations, etc that can easily be turned into llvm IR.
 * 
 ********/

pub mod arguments;
pub mod constants;
pub mod debug;
pub mod functions;
pub mod types;

#[derive(Default, Debug, Clone)]
pub struct Analysis {
    pub constants: HashMap<String, LLVMConstant>,
    pub externs: Vec<(String, Vec<LLVMArgument>, LLVMTypeWrapper)>,
    pub functions: Vec<FunctionContext>
}

pub fn analyze_root(lines: Vec<TextLine>) -> Analysis {
    // create analysis from default template
    let mut output = Analysis::default();

    // for each line, load
    for line in lines {
        // create iterator for symbols
        let mut symbols_iter = line.symbols.iter();

        // since this is a root, the first element of the line should be a statement
        let statement = symbols_iter.next();
        let statement = if statement.is_some() { statement.unwrap() } else { continue; };
        let statement = match statement {
            TextSymbol::Statement(a) => a,
            _ => panic!("Root line did not start with a statement!")
        }.as_str();

        // match statement to creation operation
        match statement {
            "const" => {
                // make sure proper length and set operation
                
                // unpack
                let name = match symbols_iter.next().unwrap() { TextSymbol::Statement(a) => a, _ => panic!("Const 2nd not a statement!") };
                let llvm_type = match symbols_iter.next().unwrap() { TextSymbol::Type(a) => a, _ => panic!("Const 3rd not a type!") };
                let _op = match symbols_iter.next().unwrap() { TextSymbol::Operation(op) => op, _ => panic!("Const 4th not an operation!") };
                let llvm_value = match symbols_iter.next().unwrap() { TextSymbol::Statement(a) => a, _ => panic!("Const 5th not a statement!") };
            
                // add const
                let value = LLVMConstant::from_string_and_value(llvm_type.clone(), llvm_value.clone());
                if value.is_some() {
                    output.constants.insert(name.clone(), value.unwrap());
                } else { println!("Constant value did not parse!") }
            },
            "extern" => {
                // todo check argument length

                // unpack
                let name = match symbols_iter.next().unwrap() { TextSymbol::Statement(a) => a, _ => panic!("Const 2nd not a statement!") };
                
                // load arguments
                let arguments = match symbols_iter.next().unwrap() { TextSymbol::TypedTuple(a) => a, _ => panic!("Const 3rd not a typed tuple!") };
                let arguments = LLVMArgument::from_tuple_list(arguments.clone());

                // load return type
                let ret_type = match symbols_iter.next().unwrap() { TextSymbol::Type(a) => a, _ => panic!("Const 4th not a statement!") };
                let ret_type = LLVMTypeWrapper::from_str(ret_type.clone());
                let ret_type = if ret_type.is_some() { ret_type.unwrap() } else { panic!("Invalid return type: {:?}", ret_type); };

                // add extern
                output.externs.push((name.clone(), arguments, ret_type));
            },
            "fun" => {
                // todo check arguments

                // unpack
                let name = match symbols_iter.next().unwrap() { TextSymbol::Statement(a) => a, _ => panic!("Function 2nd not a statement!") };
                let mut args: Vec<(String, String)> = Vec::new();
                let mut ret_type = LLVMTypeWrapper { inner: LLVMType::Void, is_pointer: false };
                let mut closure: Option<Vec<TextLine>> = None;

                // unpack 3rd
                match symbols_iter.next().unwrap() {
                    TextSymbol::TypedTuple(a) => args = a.clone(),
                    TextSymbol::Closure(a) => closure = Some(a.clone()),
                    TextSymbol::Type(a) => ret_type = LLVMTypeWrapper::from_str(a.clone()).unwrap(),
                    _ => panic!("Function 3rd is not a closure, type, or arguments!")
                }

                // unpack 4th if necessary
                if closure.is_none() || matches!(ret_type.inner, LLVMType::Void) {
                    if let Some(next) = symbols_iter.next() {
                        match next {
                            TextSymbol::Closure(a) => closure = Some(a.clone()),
                            TextSymbol::Type(a) => ret_type = LLVMTypeWrapper::from_str(a.clone()).unwrap(),
                            _ => panic!("Function 4th is not a closure or a type!")
                        }
                    }
                }

                // unpack 5th if necessary
                if closure.is_none() {
                    if let Some(next) = symbols_iter.next() {
                        match next {
                            TextSymbol::Closure(a) => closure = Some(a.clone()),
                            _ => panic!("Function 5th is not a closure!")
                        }
                    }
                }

                // make sure closure is set
                let _closure = if closure.is_some() { closure.unwrap() } else { panic!("No closure given to function!") };

                // create new function context
                let context = FunctionContext::new(name.clone(), LLVMArgument::from_tuple_list(args), ret_type);
            
                // todo process

                // save context
                output.functions.push(context);
            },
            _ => println!("Unknown operation {}", statement)
        }
    }

    // pass back
    return output;
}