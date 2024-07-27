use std::{
	path::{
		PathBuf
	}
};

use serde::{
	Serialize,
	Deserialize
};


/// todo: add more features to this struct if needed
#[derive(Serialize, Deserialize)]
pub struct Source
{
	path: PathBuf
}

impl Source
{
	pub fn new(path: &str) -> Self
	{
		Self {
			path: path.into()
		}
	}
}

impl Default for Source
{
	fn default() -> Self {
		Self::new("")
	}
}

pub type Sources = Vec<Source>;
