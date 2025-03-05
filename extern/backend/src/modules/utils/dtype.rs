use serde::{Deserialize, Serialize};
// -------------------- DTYPE -------------------- //
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum DType {
    Tensor {
        dtype: Box<DType>,
        shape: Vec<String>,
    },
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    String,
    Void,
}
