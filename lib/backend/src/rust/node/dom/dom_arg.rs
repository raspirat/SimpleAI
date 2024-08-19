use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct DomArg
{
	name: String,
	connect: Vec<(u64, String)>
}
