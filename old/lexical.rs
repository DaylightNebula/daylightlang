use crate::enums::{Statement, Operation, Value, Argument, Type};


/**
 * TODO:
 * [x] Closures
 * [x] () parsing
 * [x] Types
 * [x] Function Arguments
 * [ ] Function Inputs
 * [ ] Expressions
 * [ ] Make function calls more dependent on a known list of functions
 */

pub fn build_tree(input: &str) -> Vec<Statement> {
    // create list of segments
    let mut segments = Vec::new();

    // start looping through characters
    let mut current_parts = Vec::<&str>::new();
    let mut cur_start_idx = 0;
    let mut string_freeze = false;
    let mut curl_freeze = false;
    let mut par_freeze = false;
    for (c_idx, c) in input.chars().enumerate() {
        // if string freeze is active, handle
        if string_freeze {
            if c != '\"' { continue }
            else { string_freeze = false }
        } else if c == '\"' {
            string_freeze = true;
        }

        // if a space is found, process content and set cur start index to c index + 1
        if c == ' ' || c == '\n' || c == '(' || c == ')' || c == '{' || c == '}' {
            // detect disable par freeze
            if c == ')' { par_freeze = false; }
            if c == '}' { curl_freeze = false; }

            if !par_freeze && !curl_freeze {
                // get part
                let part = &input[cur_start_idx..c_idx];
                cur_start_idx = c_idx + 1;

                // save part
                if part != "" { current_parts.push(part); }
            }

            // detect enable par freeze
            if c == '(' { par_freeze = true; }
            if c == '{' { curl_freeze = true; }
        }

        // if end line, process parts into final statement
        if c == '\n' && !curl_freeze {
            // remove pesky /r's from last element
            let last = current_parts.last();
            let last = if last.is_some() { last.unwrap() } else { &"" };
            if last.ends_with("\r") {
                let index = current_parts.len() - 1;
                let new = &last[..last.len() - 1];

                // only add back if not blank
                if new != "" { current_parts[index] = new; } 
                else { current_parts.remove(index); }
            }

            // attempt to convert parts to segment
            let segment = convert_parts_to_segments(&current_parts);
            if segment.is_some() {
                segments.push(segment.unwrap());
            }

            // clear parts
            current_parts.clear();
        }
    }

    // return list of segments
    return segments;
}

pub fn convert_parts_to_segments(
    parts: &Vec<&str>
) -> Option<Statement> {
    // if no parts, skip
    if parts.is_empty() { return None }

    // attempt to match first part to an expression (aka segment type)
    let exp = *parts.first().unwrap();
    if exp == "" { println!("Cancelling on blank expression with parts: {:?}", parts); return None }
    Some(match exp {
        // constant conversion
        "const" => {
            // make sure enough arguments
            if parts.len() < 4 { return Some(Statement::FailedRead("Not enough arguments! Sample: const helloWorld = \"Hello World!\"".to_string())) }
            
            // get operation and make sure it is valid and set
            let operation = Operation::from_str(parts[2]);
            if operation.is_none() { return Some(Statement::FailedRead(format!("Failed to parse operation: {}", parts[2]))) }
            let operation = operation.unwrap();
            if !matches!(operation, Operation::Set) { return Some(Statement::FailedRead("Only set operation is allowed here!".into())); }

            // get value and make sure it is valid
            let value = Value::from_str(parts[3]);
            if value.is_none() { return Some(Statement::FailedRead(format!("Failed to read value: {}", parts[3]))) }

            // return constant statement
            Statement::Constant(parts[1].into(), operation, value.unwrap())
        },

        "extern" => {
            // make sure enough arguments
            if parts.len() != 5 && parts.len() != 3 { return Some(Statement::FailedRead("Not enough arguments! Sample: extern printf(text: string): i32".to_string())) }
            
            // process arguments
            let arguments = Argument::from_str_multi(parts[2]);
            if arguments.is_none() { return Some(Statement::FailedRead("Failed to parse arguments!".into())) }
            let arguments = arguments.unwrap();

            // process types
            let subtype = if parts.len() == 5 {
                Type::from_str(parts[4])
            } else { Type::Void };

            Statement::External(parts[1].into(), arguments, subtype)
        },
        
        "fun" => {
            // make sure enough arguments
            if parts.len() < 3 || parts.len() > 6 { 
                return Some(Statement::FailedRead("Not enough arguments! Sample: fun test(hello: i32): bool {".to_string()))
            }

            // inputs
            let has_args = parts.len() == 4 || parts.len() == 6;
            let args = 
                if has_args { Argument::from_str_multi(parts[2]) } 
                else { None };

            // if the function has a return type, grab it
            let return_type_index = parts.len() - 3;
            let has_return_type = parts[return_type_index] == ":";
            let return_type = if has_return_type { Type::from_str(parts[return_type_index + 1]) } else { Type::Void };

            // parse closure
            let closure = build_tree(parts.last().unwrap());

            Statement::Function(parts[1].into(), args, return_type, closure)
        },

        "let" | "var" => Statement::FailedRead("Create var expression converion".to_string()),
        _ => {
            // for now, assume this is a function call
            if parts.len() < 1 { return Some(Statement::FailedRead("Failed to parse function call.  To few arguments".into())) }

            // build inputs list if this has inputs
            let has_inputs = parts.len() >= 2;
            let inputs = 
                if has_inputs {
                    Some(parts[1..].to_vec().iter().map(|a| a.to_string()).collect::<Vec<String>>())
                } else { None };

            // add call function statement
            Statement::CallFunc(parts.first().unwrap().to_string(), inputs)
        }
    })
}
