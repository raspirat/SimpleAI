use serde::{Deserialize, Serialize};
use crate::node::dom::dom_arg::DomArg;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct DomNode
{
	id: u64,
	args: DomArg,
	kind: String
}
