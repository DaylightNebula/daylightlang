use crate::enums::Statement;


/**
 * Code could be split into a few different things
 * - Keywords (let, const, extern, fun, etc)
 * - Names (helloWorld for example)
 * - Operations (+, -, +=, etc)
 * - Type (i32, string, etc)
 * - Typed Objects (Strings, Floats, etc)
 * - Closure (the things {} surrounding code)
 * - Typed tuple (input arguments to a function)
 * - Untyped tuple (list of objects that are used to call a function)
 */

pub fn build_tree(input: &str) -> Vec<Statement> {
    // create list of segments
    let mut segments = Vec::new();

    // start looping through characters
    let mut current_parts = Vec::<&str>::new();
    let mut cur_start_idx = 0;
    let mut string_freeze = false;
    for (c_idx, c) in input.chars().enumerate() {
        // if string freeze is active, handle
        if string_freeze {
            if c != '\"' { continue }
            else { string_freeze = false }
        } else if c == '\"' {
            string_freeze = true;
        }

        // if a space is found, process content and set cur start index to c index + 1
        if c == ' ' || c == '\n' {
            // get part
            let part = &input[cur_start_idx..c_idx];
            cur_start_idx = c_idx + 1;

            // save part
            if part != "" { current_parts.push(part); }
        }

        // if end line, process parts into final statement
        if c == '\n' {
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
        "const" => Statement::Todo("Constant converion".to_string()),
        "extern" => Statement::Todo("Extern converion".to_string()),
        "fun" => Statement::Todo("Function expression converion".to_string()),
        "let" | "var" => Statement::Todo("Create var expression converion".to_string()),
        "}" => Statement::Todo("Handle closures!".to_string()),
        _ => Statement::Todo(format!("What to do if no expression found! \"{}\"", exp))
    })
}
