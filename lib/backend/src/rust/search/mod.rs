use std::{
	collections::{
		BinaryHeap
	},
	path::{
		PathBuf
	}
};

use crate::{
	info::{
		Info,
		Infos
	},
	errors::{
		*
	},
	node::{
		Node,
		Nodes,
		to_infos,
		get_nodes
	}
};

use boyer_moore_magiclen::{
	BMByte
};

use serde::{
	Serialize
};

#[derive(Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct SearchResult(usize, Info);
pub type SearchResults = BinaryHeap<SearchResult>;

pub struct Search
{
	infos: Infos
}

impl Search
{
	pub fn search(&self, query: &str) -> Option<SearchResults>
	{
		let mut sr: Option<SearchResults> = None;
		let bmbo: Option<BMByte> = BMByte::from(query);
		if bmbo.is_some()
		{
			let bmb: BMByte = bmbo.unwrap();
			for info in &self.infos
			{
				let text: String = serde_json::to_string(&info).unwrap();
				let res: usize = bmb.find_full_all_in(text).len();
				let i: SearchResult = SearchResult(res, info.clone());
				sr.get_or_insert_with(|| SearchResults::new()).push(i);
			}
		}
		sr
	}

	pub fn new() -> Result<Self, Error>
	{
		Ok(
			Self::from(get_nodes()?)
		)
	}

	// pub fn new(path: PathBuf) -> Result<Search, Error> {
	// 	let mut infos: Infos = vec![];
	// 	let dir = path.read_dir();
	// 	if dir.is_err() {
	// 		return Err(
	// 			Error::FileNotFoundError(
	// 				Args::new(
	// 					ErrorTypes::Critical,
	// 					"Creating Search Object",
	// 					 path.to_str().unwrap(),
	// 					"Nodes folder not found",
	// 					"This isn't normal, recreating..."
	// 				)
	// 			)
	// 		)
	// 	};
	// 	let dir = dir.unwrap();
	// 	for rdobj in dir
	// 	{
	// 		if rdobj.is_ok()
	// 		{
	// 			let path = rdobj.unwrap().path();
	// 			if path.is_dir()
	// 			{
	// 				infos.push(Info::try_from(path).unwrap());
	// 			}
	// 		}
	// 	}
	// 	Ok(
	// 		Self {
	// 			infos
	// 		}
	// 	)
	// }
}

impl From<Infos> for Search
{
	fn from(infos: Infos) -> Self {
		Self {
			infos
		}
	}
}

impl From<Nodes> for Search
{
	fn from(nodes: Nodes) -> Self {
		Self::from(to_infos(nodes))
	}
}

