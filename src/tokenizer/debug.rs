use super::structs::*;

pub fn convert_lines_to_string(lines: &Vec<TextLine>, tabs: usize) -> String {
    let mut output: Vec<String> = Vec::new();

    for line in lines {
        let mut final_line = Vec::<String>::new();

        for symbol in &line.symbols {
            let part = match symbol {
                TextSymbol::Statement(a) => format!("STATEMENT({})", a),
                TextSymbol::Operation(op) => format!("OPERATION({})", op.to_str()),
                TextSymbol::Type(t) => format!("TYPE({})", t),
                TextSymbol::ArrayDeclaration(_) => todo!("Array debug build not created!"),
                TextSymbol::UnTypedTuple(a) => format!("UNTYPED({})", a.join(", ")),
                TextSymbol::TypedTuple(a) => format!("TYPED({})", a.iter().map(|(a, b)| format!("{:?}: {:?}", a, b)).collect::<Vec<String>>().join(", ")),
                TextSymbol::Generics(a) => format!("GENERICS({})", a.iter().map(|(a, b)| format!("{:?}: {:?}", a, b)).collect::<Vec<String>>().join(", ")),
                TextSymbol::Closure(a) => format!("CLOSURE(\n{}\n)", convert_lines_to_string(a, tabs + 1)),
                TextSymbol::Comment(a) => format!("COMMENT({})", a),
            };
            final_line.push(part);
        }

        output.push(format!("{}{}", "    ".repeat(tabs), final_line.join(", ")));
    }

    return output.join("\n");
}