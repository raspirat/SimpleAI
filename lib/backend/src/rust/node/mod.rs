use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::PathBuf;

use serde::{
	Deserialize,
	Serialize
};

use crate::{
	errors::{*},
	util::{
		default_hash::{
			DefaultHash
		}
	},
	stdpaths::{*},
	info::{*}
};

mod includes;
mod node_decl;
mod data_type;
mod arg;
mod dom;

use includes::{*};
use node_decl::{*};
use data_type::{*};
use arg::{*};
use dom::{
	*,
	dom_arg::{*},
	dom_node::{*}
};

#[derive(Hash, Eq, PartialEq, DefaultHash)]
pub struct Node
{
	pub node_decl: NodeDecl,
	pub node_dom: Dom
}

impl TryFrom<(Info, Nodes)> for Node
{
	type Error = Error;
	fn try_from(value: (Info, Nodes)) -> Result<Self, Self::Error>
	{
		let (info, nodes) = value;
		for node in nodes
		{
			if node.node_decl.info.default_hash() == info.default_hash()
			{
				return Ok(node)
			}
		}
		Err(
			Error::NodeConversionError(
				Args::new(
					ErrorTypes::Default,
					"Converting Info to Node",
					info.name().as_str(),
					"Couldn't convert Info to Node (No matching Info found for Nodes)",
					""
				)
			)
		)
	}
}

impl TryFrom<Info> for Node
{
	type Error = Error;

	fn try_from(info: Info) -> Result<Self, Self::Error> {
		Self::try_from(
				(info, get_nodes()?)
		)
	}
}

impl TryFrom<PathBuf> for Node
{
	type Error = Error;

	fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
		Ok(
			Self {
				node_decl: NodeDecl::try_from(path.clone())?,
				node_dom: Dom::try_from(path)?
			}
		)
	}
}

pub type Nodes = HashSet<Node>;

pub fn get_nodes() -> Result<Nodes, Error>
{
	let mut set: Nodes = HashSet::new();
	for dir_os_str in node_path().iter()
	{
		let mut dir: PathBuf = dir_os_str.into();
		if dir.is_dir()
		{
			let res = Node::try_from(dir);
			match res {
				Ok(node) => {
					set.insert(node);
				}
				Err(err) => {
					match err {
						Error::FileNotFoundError(_) => { return Err(err) }
						Error => {} // todo: implement a notification button/page where this is showed
					}
				}
			}
		}
	}
	Ok(set)
}

pub fn to_infos(nodes: Nodes) -> Infos
{
	nodes
		.iter()
		.map(|node: &Node| {
			node.node_decl.info.clone()
		})
		.collect::<Infos>()
}
