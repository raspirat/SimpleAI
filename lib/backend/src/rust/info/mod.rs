use std::{
	fs::{
		read_to_string
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

#[derive(Serialize, Deserialize)]
pub struct Info
{
	name: String,
	version: String,
	description: String,
	author: String,
	keywords: Vec<String>,
	website: Url
}

impl Info
{
	pub fn new(
		name: &str,
		version: &str,
		description: &str,
		author: &str,
		keywords: Vec<&str>, // not sure if works
		website: Url
	) -> Self
	{
		Self {
			name: name.into(),
			version: version.into(),
			description: description.into(),
			author: author.into(),
			keywords: keywords.into(),
			website
		}
	}
}

impl Default for Info
{
	fn default() -> Self {
		Self::new(
			"Default",
			"1.0.0",
			"The default Information",
			"sert",
			vec!["default"],
			Url::from_str("https://github.com/sertschgi").unwrap()
		)
	}
}

impl From<PathBuf> for Info
{
	fn from(path: PathBuf) -> Self {
		let fobj = read_to_string(path.join("init.toml"));
		if fobj.is_ok()
		{
			let sfobj = fobj.unwrap();
			return toml::from_str(sfobj.into()).unwrap_or(Self::default());
		}
		return Self::default();
	}
}

pub type Infos = Vec<Info>;