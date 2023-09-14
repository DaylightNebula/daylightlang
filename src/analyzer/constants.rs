

#[derive(Debug, Clone)]
pub enum LLVMConstant {
    I8(i8), 
    I16(i16), 
    I32(i32), 
    Str(String, usize), // string and size (size as seen by LLVM)
    // todo populate
}

impl Default for LLVMConstant {
    fn default() -> Self {
        Self::I8(0)
    }
}

impl LLVMConstant {
    pub fn from_string_and_value(input_type: String, input: String) -> Option<Self> {
        match input_type.as_str() {
            "str" => {
                // get chars iterator
                let mut chars = input.chars();

                // make sure input does not start and end with a "
                let mut input = input.clone();
                if chars.next().unwrap() == '\"' && chars.last().unwrap() == '\"' {
                    input = input[1..input.len() - 1].to_string();
                }

                // return final string
                Some(Self::Str(input.clone(), input.len()))
            },
            "i8" => {
                let value = input.parse::<i8>();
                if value.is_ok() { Some(Self::I8(value.unwrap())) } else { None }
            },
            "i16" => {
                let value = input.parse::<i16>();
                if value.is_ok() { Some(Self::I16(value.unwrap())) } else { None }
            },
            "i32" => {
                let value = input.parse::<i32>();
                if value.is_ok() { Some(Self::I32(value.unwrap())) } else { None }
            },
            _ => None
        }
    }

    pub fn to_llvm_type_str(&self) -> String {
        match self {
            Self::I8(_) => "i8".to_string(),
            Self::I16(_) => "i16".to_string(),
            Self::I32(_) => "i32".to_string(),
            Self::Str(_, len) => format!("[{} x i8]", len)
        }
    }

    pub fn to_llvm_value(&self) -> String {
        match self {
            Self::I8(a) => a.to_string(),
            Self::I16(a) => a.to_string(),
            Self::I32(a) => a.to_string(),
            Self::Str(a, _) => format!("c\"{}\"", a),
        }
    }
}