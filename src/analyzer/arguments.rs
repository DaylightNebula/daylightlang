use super::types::LLVMTypeWrapper;

#[derive(Debug, Clone)]
pub struct LLVMArgument {
    pub name: String,
    pub type_wrapper: LLVMTypeWrapper
}

impl LLVMArgument {
    pub fn from_tuple(name: String, d_type: String) -> Option<Self> {
        let llvm_type = LLVMTypeWrapper::from_str(d_type);
        if llvm_type.is_some() {
            Some(Self { name, type_wrapper: llvm_type.unwrap() })
        } else { None }
    }

    pub fn from_tuple_list(vec: Vec<(String, String)>) -> Vec<Self> {
        let mut output = Vec::with_capacity(vec.len());

        for (name, d_type) in vec {
            let wrapper = Self::from_tuple(name, d_type);
            if wrapper.is_some() {
                output.push(wrapper.unwrap());
            }
        }

        return output;
    }

    pub fn list_to_llvm(list: &Vec<LLVMArgument>) -> String {
        list.iter().map(|a| a.type_wrapper.to_str()).collect::<Vec<String>>().join(", ")
    }
}