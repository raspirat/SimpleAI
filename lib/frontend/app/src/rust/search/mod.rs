use std::{
	path::{
		PathBuf
	}
};

use libbacksne::{
	info::{
		Info,
		Infos
	},
	node::{
		Node
	}
};

pub struct Search
{
	infos: Infos
}

impl Search
{
	pub fn search(self, query: &str) -> Infos
	{
		todo!();
	}
}

impl From<PathBuf> for Search
{
	fn from(path: PathBuf) -> Self {
		let mut infos: Infos = vec![];
		for rdobj in path.read_dir().unwrap()
		{
			if rdobj.is_ok()
			{
				let path = rdobj.unwrap().path();
				if path.is_dir()
				{
					infos.push(Info::from(path));
				}
			}
		}
		Self {
			infos
		}
	}
}