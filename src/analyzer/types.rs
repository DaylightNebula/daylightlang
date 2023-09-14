#[derive(Debug, Clone)]
pub struct LLVMTypeWrapper {
    pub inner: LLVMType,
    pub is_pointer: bool
}

impl LLVMTypeWrapper {
    pub fn to_str(&self) -> String {
        match self.is_pointer {
            true => format!("{}*", self.inner.to_str()),
            false => format!("{}", self.inner.to_str())
        }
    }

    pub fn from_str(input: String) -> Option<Self> {
        // check if pointer
        let is_pointer = input.ends_with("*");

        // get input
        let input = if is_pointer { &input[.. input.len() - 1] } else { &input[.. input.len()] };

        // convert inner from string and return self
        let inner = LLVMType::from_str(input);
        if inner.is_some() {
            Some(Self { inner: inner.unwrap(), is_pointer })
        } else { None }
    }
}

#[derive(Debug, Clone)]
pub enum LLVMType {
    I8, I16, I32,
    Str, Wildcard, Void
}

impl LLVMType {
    pub fn to_str(&self) -> &str {
        match self {
            LLVMType::I8  => "i8",
            LLVMType::I16 => "i16",
            LLVMType::I32 => "i32",
            LLVMType::Str => "i8",
            LLVMType::Wildcard => "...",
            LLVMType::Void => "void"
        }
    }

    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "i8" => Some(Self::I8),
            "i16" => Some(Self::I16),
            "i32" => Some(Self::I32),
            "str" => Some(Self::Str),
            "..." => Some(Self::Wildcard),
            "void" => Some(Self::Void),
            _ => None
        }
    }
}