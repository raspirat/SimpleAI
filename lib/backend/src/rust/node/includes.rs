use std::{
	collections::{
		BTreeMap
	},
	path::{
		PathBuf
	}
};

use serde::{
	Serialize,
	Deserialize
};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub(crate) struct IncludeOptions
{
	version: String,
	path: Option<PathBuf>
}

pub type Includes = BTreeMap<String, IncludeOptions>;

fn include(includes: Includes)
{
	for (name, include) in includes
	{
		todo!() // implement this
	}
}

