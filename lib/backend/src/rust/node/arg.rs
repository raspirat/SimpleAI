use serde::{Deserialize, Serialize};
use crate::node::data_type::DataType;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct Arg
{
	name: String,
	dtype: DataType,
	desc: String,
}
