#[derive(Debug)]
pub enum Statement {
    Constant(String, Operation, Value), 
    External, 
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