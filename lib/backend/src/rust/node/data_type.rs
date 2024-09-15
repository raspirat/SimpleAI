use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub enum DataType {
    Str,
    Bool,
    Int(i8),
    Float(i8),
    Tensor { kind: String, dtype: Box<DataType> },
}
