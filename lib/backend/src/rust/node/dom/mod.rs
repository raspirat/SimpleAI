pub mod dom_arg;
pub mod dom_node;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use dom_node::{*};
use crate::errors::Error;
use crate::util::try_from_json;
use crate::stdpaths::NODE_DOM_FILE_NAME;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Dom
{
	elements: Vec<DomNode>
}

impl TryFrom<PathBuf> for Dom
{
	type Error = Error;

	fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
		let dom_path: PathBuf = path.join(NODE_DOM_FILE_NAME);
		try_from_json(dom_path)
	}
}