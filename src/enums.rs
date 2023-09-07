#[derive(Debug)]
pub enum Statement {
    Constant(String, Operation, Value), 
    External, 
    Function, 
    CreateVar,
    Todo(String)
}

#[derive(Debug)]
pub enum Operation {
    Add, Subtract, Multiply, Set
}

#[derive(Debug)]
pub enum Value {
    String,
    I32,
    // TODO implement rest
}