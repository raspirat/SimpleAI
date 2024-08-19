use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::errors::Error;
use crate::info::Info;
use crate::node::Arg;
use crate::node::includes::Includes;
use crate::util::{*};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct NodeDecl
{
	pub favourite: bool,
	pub includes: Includes,
	pub info: Info,
	pub inputs: Vec<Arg>,
	pub outputs: Vec<Arg>
}

impl TryFrom<PathBuf> for NodeDecl
{
	type Error = Error;

	fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
		try_from_json(value)
	}
}
