use std::collections::HashMap;

use crate::tokenizer::structs::*;

/********
 * 
 * This module turns the text lines from the tokenizer into final lists of
 * structs, functions, operations, etc that can easily be turned into llvm IR.
 * 
 ********/

#[derive(Default, Debug, Clone)]
pub struct Analysis {
    pub constants: HashMap<String, LLVMValue>
}

#[derive(Debug, Clone)]
pub enum LLVMValue {
    I8(i8), 
    I16(i16), 
    I32(i32), 
    Str(String, usize), // string and size (size as seen by LLVM)
    // todo populate
}

impl Default for LLVMValue {
    fn default() -> Self {
        Self::I8(0)
    }
}

impl LLVMValue {
    pub fn from_string_and_value(input_type: String, input: String) -> Option<Self> {
        match input_type.as_str() {
            "str" => {
                // get chars iterator
                let mut chars = input.chars();

                // make sure input does not start and end with a "
                let mut input = input.clone();
                if chars.next().unwrap() == '\"' && chars.last().unwrap() == '\"' {
                    input = input[1..input.len() - 1].to_string();
                }

                // return final string
                Some(LLVMValue::Str(input.clone(), input.len()))
            },
            "i8" => {
                let value = input.parse::<i8>();
                if value.is_ok() { Some(LLVMValue::I8(value.unwrap())) } else { None }
            },
            "i16" => {
                let value = input.parse::<i16>();
                if value.is_ok() { Some(LLVMValue::I16(value.unwrap())) } else { None }
            },
            "i32" => {
                let value = input.parse::<i32>();
                if value.is_ok() { Some(LLVMValue::I32(value.unwrap())) } else { None }
            },
            _ => None
        }
    }

    pub fn to_llvm_type_str(&self) -> String {
        match self {
            LLVMValue::I8(_) => "i8".to_string(),
            LLVMValue::I16(_) => "i16".to_string(),
            LLVMValue::I32(_) => "i32".to_string(),
            LLVMValue::Str(_, len) => format!("[{} x i8]", len)
        }
    }

    pub fn to_llvm_value(&self) -> String {
        match self {
            LLVMValue::I8(a) => a.to_string(),
            LLVMValue::I16(a) => a.to_string(),
            LLVMValue::I32(a) => a.to_string(),
            LLVMValue::Str(a, _) => format!("c\"{}\"", a),
        }
    }
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
                let value = LLVMValue::from_string_and_value(llvm_type.clone(), llvm_value.clone());
                if value.is_some() {
                    output.constants.insert(name.clone(), value.unwrap());
                } else { println!("Constant value did not parse!") }
            },
            _ => println!("Unknown operation {}", statement)
        }
    }

    // pass back
    return output;
}