use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub enum DataType
{
	str,
	bool,
	int(i8),
	float(i8),
	tensor {
		kind: String,
		dtype: Box<DataType>
	}
}
