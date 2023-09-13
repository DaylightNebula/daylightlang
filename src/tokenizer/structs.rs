#[derive(Debug, Clone)]
pub struct TextLine {
    pub symbols: Vec<TextSymbol>
}

#[derive(Debug, Clone)]
pub enum TextSymbol {
    Statement(String),
    Operation(Operation),
    Type(String),
    ArrayDeclaration(i32),
    UnTypedTuple(Vec<String>),
    TypedTuple(Vec<(String, String)>),
    Generics(Vec<(String, String)>),
    Closure(Vec<TextLine>),
    Comment(String)
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    AddEquals,
    Subtract,
    SubtractEquals,
    Multiply,
    MultiplyEquals,
    Divide,
    DivideEquals,
    Set,
    Equals,
    NotEquals,
    And,
    Or
}

impl Operation {
    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "+" => Some(Self::Add),
            "+=" => Some(Self::AddEquals),
            "-" => Some(Self::Subtract),
            "-=" => Some(Self::SubtractEquals),
            "*" => Some(Self::Multiply),
            "*=" => Some(Self::MultiplyEquals),
            "/" => Some(Self::Divide),
            "/=" => Some(Self::DivideEquals),
            "=" => Some(Self::Set),
            "==" => Some(Self::Equals),
            "!=" => Some(Self::NotEquals),
            "&&" => Some(Self::And),
            "||" => Some(Self::Or),
            _ => None
        }
    }
}