use std::*;
use std::path::PathBuf;

use crate::errors;

/// Theme struct with properties:
///  - `background` _(type: __Background__)_
#[derive(serde::Deserialize)]
pub struct Theme
{
	background: Background,
}

impl Theme
{
	/// Loads a theme from a specific path.
	pub fn load()
	{
		todo!();
	}
}

impl From<&str> for Theme
{
	fn from(path: &str) -> Self
	{
		Self::from(
			PathBuf::from(path)
		)
	}
}

impl From<PathBuf> for Theme
{
	fn from(path: PathBuf) -> Self
	{
		serde_json::from_reader(
			fs::File::create(
				path.join("init.json")
				    .to_str()
				    .unwrap()
			).unwrap()
		).unwrap()
	}

}

/// Background image struct with properties:
///  - `url` _(type: __str__)_
///  - `scale` _(type: __f32__)_
#[derive(serde::Deserialize)]
struct Image
{
	url: String,
	scale: f32,
}


/// Background enum, in which you can choose between:
/// - `Color` _(type: __str__)_
/// - `Image` _(type: __Image__)_
#[derive(serde::Deserialize)]
enum Background
{
	Color(String),
	Image(Image),
}
