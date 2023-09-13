use self::structs::*;

pub mod structs;

pub fn breakup_text(content: &str, debug: bool) -> Vec<TextLine> {
    let mut output = Vec::new();

    // setup some variables
    let mut symbols = Vec::<TextSymbol>::new();
    let mut counter = 0;
    let max_count = content.len();
    let mut cur_seg_start = 0;

    // start looping through each "line" of the file
    while counter < max_count {
        // get current character
        let c = content[counter..counter + 1].chars().next().unwrap();

        // if this is the start of a comment, build comment and skip to the end
        if c == '/' && content[counter - 1..counter].chars().next().unwrap_or(' ') == '/' {
            // advance to the end of a commend
            let to_add = depth_aware_search(content, counter + 2, '\n', None) + 2;

            // process and add comment
            let mut comment = content[counter + 2 .. counter + to_add].to_string();
            if comment.ends_with("\r") { comment.remove(comment.len() - 1); }
            symbols.push(TextSymbol::Comment(comment));

            // update counters and skip
            counter += to_add;
            cur_seg_start = counter;
            continue;
        }

        // if character is :, grab type
        if c == ':' {
            let to_add = depth_aware_search(content, counter + 2, ' ', None) + 2;
            
            // process and add statement and type
            let section = content[cur_seg_start .. counter + to_add].to_string();
            let mut sections = section.split(": ").into_iter();
            let statement = sections.next().unwrap().to_string();
            if !statement.is_empty() { symbols.push(TextSymbol::Statement(statement)); }
            symbols.push(TextSymbol::Type(sections.last().unwrap().to_string()));

            // add to counter
            counter += to_add;
            cur_seg_start = counter;

            continue;
        }

        // if character is a (, handle tuple creation
        if c == '(' {
            // run search for end of tuple
            let to_add = depth_aware_search(content, counter + 1, ')', Some('(')) + 2;
            
            // get text section and parse into symbols
            let name = content[cur_seg_start .. counter].to_string();
            if !name.is_empty() { symbols.push(TextSymbol::Statement(name)); }
            let section = content[counter + 1 .. counter + to_add - 1].to_string();

            // process the tuple
            let is_typed = section.contains(":");
            let sections = section.split(", ");
            match is_typed {
                true => {
                    let mut list: Vec<(String, String)> = Vec::new();

                    // convert each part of the section into a typed pair
                    for part in sections {
                        let parts = part.split(": ").collect::<Vec<&str>>();
                        list.push((
                            parts.first().unwrap().to_string(), 
                            parts.last().unwrap().to_string()
                        ));
                    }

                    // add to symbols
                    symbols.push(TextSymbol::TypedTuple(list));
                }
                false => {
                    // convert sectionst to a list and add symbol
                    let sections = sections.collect::<Vec<&str>>();
                    let sections = sections.iter().map(|a| a.to_string()).collect::<Vec<String>>();
                    symbols.push(TextSymbol::UnTypedTuple(sections));
                }
            }

            // counters and skip
            counter += to_add;
            cur_seg_start = counter;
            continue;
        }

        // if character is a {, handle closure creation
        if c == '{' {
            let to_add = depth_aware_search(content, counter + 1, '}', Some('{')) + 2;

            // decode section
            let section = &content[cur_seg_start + 1 .. counter + to_add - 1];
            let lines = breakup_text(section, false);
            symbols.push(TextSymbol::Closure(lines));

            // counters and skip
            counter += to_add;
            cur_seg_start = counter;
            continue;
        }

        // if character is a <, handle generics creation
        if c == '<' {
            let to_add = depth_aware_search(content, counter + 1, '>', Some('<')) + 2;

            // process name and get generic segment
            let name = content[cur_seg_start .. counter].to_string();
            if !name.is_empty() { symbols.push(TextSymbol::Statement(name)); }
            let section = content[counter + 1 .. counter + to_add - 1].to_string();

            // process generics
            let mut list = Vec::<(String, String)>::new();
            for part in section.split(", ") {
                let parts = part.split(": ").collect::<Vec<&str>>();
                list.push((parts.first().unwrap().to_string(), parts.last().unwrap().to_string()));
            }
            symbols.push(TextSymbol::Generics(list));

            // counters and skip
            counter += to_add;
            cur_seg_start = counter;
            continue;
        }

        // if this char is the end of a segment, save
        if c == ' ' || c == '\n' {
            // get segment
            let mut segment = &content[cur_seg_start..counter];

            // if segment blank, update segment counter and skip
            if segment.len() == 0 || segment == " " {
                cur_seg_start = counter + 1;
            } else {            
                // remove /r if needed
                if segment.chars().last().unwrap() == '\r' {
                    segment = &segment[..segment.len() - 1];
                }

                cur_seg_start = counter + 1;

                // attempt to convert to operation
                let operation = Operation::from_str(segment);

                // process and save segment
                if operation.is_some() {
                    symbols.push(TextSymbol::Operation(operation.unwrap()));
                } else {
                    symbols.push(TextSymbol::Statement(segment.to_string()));
                }
            }
        }

        // check if an endline was reached (;, or \n) or last character
        if c == ';' || c == '\n' {
            // filter symbols (mostly for empty symbols that make above functions easier)
            symbols.retain(|a| match a {
                TextSymbol::Statement(a) => !a.is_empty(),
                TextSymbol::UnTypedTuple(a) => if a.is_empty() { false } else { a.iter().all(|a| !a.is_empty()) }
                _ => true
            });

            // save text line
            if !symbols.is_empty() {
                if debug { println!("Adding: {:?}", symbols); }
                output.push(TextLine { symbols: symbols.clone() });
            }
            symbols.clear();
        }
        counter += 1;
    }

    return output
}

pub fn depth_aware_search(
    content: &str, 
    start: usize, 
    looking_for: char, 
    lvl_deeper: Option<char>
) -> usize {
    // setup some vars
    let mut depth = 1;
    let mut to_add = 0;

    // loop until depth is 0
    while depth > 0 {
        // get character
        let char = content.chars().nth(start + to_add);
        let char = if char.is_some() { char.unwrap() } else { depth = 0; continue; };

        // mod depth
        if char == looking_for { depth -= 1; }
        else if Some(char) == lvl_deeper { depth += 1; }

        // add to counter
        to_add += 1;
    }

    return to_add - 1;
}