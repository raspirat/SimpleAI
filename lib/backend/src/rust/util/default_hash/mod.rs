pub use default_hash_derive::{DefaultHash};
pub use std::{
	hash::{
		DefaultHasher,
		Hash,
		Hasher
	}
};

pub trait DefaultHash
{
	fn default_hash(&self) -> u64;
}