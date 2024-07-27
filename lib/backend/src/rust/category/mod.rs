use serde::{
	Serialize,
	Deserialize
};


/// todo: add more features to this struct if needed
#[derive(Serialize, Deserialize)]
pub struct Category
{
	name: String
}

impl Category
{
	pub fn new(name: &str) -> Self { Self { name: name.into() } }
	pub fn name(self) -> String { self.name }
}

impl Default for Category
{
	fn default() -> Self { Self::new("Default") }
}

pub type Categories = Vec<Category>;