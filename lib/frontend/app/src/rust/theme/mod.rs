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

use serde::{
	Serialize,
	Deserialize
};
use url::Url;
use libbacksne::{
	category::{
		Categories
	},
	info::{
		Info
	},
	sources::{
		Sources
	}
};
use libbacksne::category::Category;
use libbacksne::sources::Source;

#[derive(Serialize, Deserialize)]
struct Theme
{
	categories: Categories,
	info: Info,
	sources: Sources
}

impl Theme
{
	pub fn new(categories: Categories, info: Info, sources: Sources) -> Self {
		Self {
			categories,
			info,
			sources
		}
	}
}

impl Default for Theme
{
	fn default() -> Self {
		 Self::new(
			 vec!(Category::default()),
			  Info::new(
				  "DefaultTheme",
				  "1.0.0",
				  "This is the default theme.",
				  "sert",
				  Url::from_str("https://github.com/sertschgi").unwrap()
			  ),
			  vec!(Source::default())
		 )
	}
}

impl From<PathBuf> for Theme
{
	fn from(path: PathBuf) -> Self {
		let fobj = read_to_string(path);
		if fobj.is_ok()
		{
			let sfobj = fobj.unwrap();
			return toml::from_str(sfobj.into()).unwrap_or(Self::default());
		}
		return Self::default();
	}
}