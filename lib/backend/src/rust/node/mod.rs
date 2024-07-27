use serde::{
	Serialize,
	Deserialize
};

use crate::{
	info::{
		Info
	},
	category::{
		Category
	},
	includes::{
		Includes
	}
};

/// This is the node that defines the node editor
#[derive(Serialize, Deserialize)]
pub struct Node
{
	category: Category,
	favourite: bool,
	includes: Includes,
	info: Info
}