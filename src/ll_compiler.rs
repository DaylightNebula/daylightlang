use crate::enums::{Statement, Type};

pub fn compile(statements: Vec<Statement>) -> Vec<String> {
    // create output list
    let mut output = Vec::new();

    // for each statement, generate llvm IR code
    for statement in statements {
        output.push(compile_single(statement));
    }

    // return final output
    return output;
}

pub fn compile_single(statement: Statement) -> String {
    match statement {
        Statement::Constant(name, _, value) => {
            format!("@.{} = private unnamed_addr constant {} {}", name, value.type_str(), value.value_str())
        },

        Statement::External(name, args, subtype) => {
            format!(
                "declare {} @{}({}) nounwind", 
                subtype.to_str(), 
                name, 
                args.iter().map(|a| a.to_extern_func_args()).collect::<Vec<String>>().join(", ")
            )
        },

        Statement::Function(name, args, subtype, closure) => {
            // compile contents
            let mut content = compile(closure);
            if name == "main" { content.push("ret i32 0".to_string()); }
            let content = content.join("\n");

            // type
            let mut subtype = subtype;
            if matches!(subtype, Type::Void) && name == "main" { subtype = Type::I32; }
            
            // format final
            format!(
                "define {} @{}() {{\n{}\n}}",
                subtype.to_str(),
                name,
                content
            )
        },

        Statement::CallFunc(name, inputs) => {
            let contents = if inputs.is_some() {
                inputs.unwrap().iter().map(|a| format!("ptr @.{}", a))
                    .collect::<Vec<String>>().join(", ")
            } else { String::default() };

            format!(
                "call i32 @{}({})",
                name,
                contents
            )
        },

        Statement::FailedRead(error) => panic!("Compile failed with error: {}", error),
    }
}
