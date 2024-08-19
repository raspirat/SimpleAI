use std::{
	collections::{
		HashSet
	},
	fs::{
		read_to_string
	},
	hash::{
		Hash,
		Hasher,
		DefaultHasher
	},
	path::{
		PathBuf
	},
	str::{
		FromStr
	}
};

use url::{
	Url
};

use serde::{
	Serialize,
	Deserialize
};

use crate::{
	util::{
		default_hash::{DefaultHash},
		*
	},
	errors::{*}
};

#[derive(Serialize, Deserialize, DefaultHash, Eq, Ord, PartialEq, PartialOrd, Clone, Hash)]
pub struct Info
{
	name: String,
	version: String,
	description: String,
	author: String,
	keywords: Vec<String>,
	website: Url,
	categories: Vec<String>
}

impl Info
{
	pub fn new(
		name: &str,
		version: &str,
		description: &str,
		author: &str,
		keywords: Vec<&str>,
		website: Url,
		categories: Vec<&str>
	) -> Self
	{
		Self {
			name: name.into(),
			version: version.into(),
			description: description.into(),
			author: author.into(),
			keywords: keywords.iter().map(|&s| s.to_string()).collect(),
			website,
			categories: categories.iter().map(|&s| s.to_string()).collect()
		}
	}

	pub fn name(&self) -> String
	{
		self.clone().name
	}
}

impl TryFrom<PathBuf> for Info
{
	type Error = Error;

	fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
	//	try_from_json(value)
		todo!();
	}
}

pub type Infos = HashSet<Info>;