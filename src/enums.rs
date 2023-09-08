#[derive(Debug)]
pub enum Statement {
    Constant(String, Operation, Value), 
    External(String, Vec<Argument>, Option<Type>), 
    Function, 
    CreateVar,
    FailedRead(String)
}

// represents mathematical operations
#[derive(Debug)]
pub enum Operation {
    Add, Subtract, Multiply, Divide, Set
}

#[derive(Debug)]
pub enum Value {
    String(String),
    I32(i32),
    // TODO implement rest
}

#[derive(Debug)]
pub enum Type {
    String,
    I32,
    // TODO implement rest
}

#[derive(Debug)]
pub struct Argument {
    index: usize,
    name: String,
    subtype: Type
}

// add functions to make working with operations easier
impl Operation {
    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "=" => Some(Operation::Set),
            "+" => Some(Operation::Add),
            "-" => Some(Operation::Subtract),
            "*" => Some(Operation::Multiply),
            "/" => Some(Operation::Divide),
            _ => None
        }
    }
}

// add functions to make working with values easier
impl Value {
    // converts a string to a value, returning non if fails
    pub fn from_str(input: &str) -> Option<Self> {
        // get last char as that denotes type
        let c = input.chars().last().unwrap_or(' ');
        match c {
            // strings
            '\"' => Some(Value::String(input[1..input.len() - 1].to_string())),

            // if not anything else, attempt to convert to int before returning nothing
            _ => {
                // attempt to parse into a i32, otherwise, return none
                let num = input.parse::<i32>();
                if num.is_ok() {
                    Some(Value::I32(num.unwrap()))
                } else { None }
            }
        }
    }
}

impl Type {
    // convert from string
    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "string" => Some(Type::String),
            "i32" => Some(Type::I32),
            _ => None
        }
    }
}

impl Argument {
    pub fn from_str(index: usize, input: &str) -> Option<Self> {
        // split argument
        let parts: Vec<&str> = input.split(": ").collect();

        // if not enough or too many arguments, error
        if parts.len() != 2 || parts.iter().any(|a| a == &"") { return None }

        // get type
        let subtype = Type::from_str(*parts.last().unwrap());
        let subtype = if subtype.is_some() { subtype.unwrap() } else { return None };

        // return final argument
        Some(Self { index, name: parts[0].into(), subtype })
    }

    pub fn from_str_multi(input: &str) -> Option<Vec<Argument>> {
        let parts = input.split(", ");
        let mut output = Vec::new();
        for (index, part) in parts.into_iter().enumerate() {
            let arg = Self::from_str(index, part);
            if arg.is_some() { output.push(arg.unwrap()); }
            else { return None; }
        }
        return Some(output)
    }
}
