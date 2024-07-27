use std::error::Error;
use std::*;
use json;

use crate::errors;

/// theme struct with properties:
///  - `background` _(type: __Background__)_
struct Theme
{
	background: Background,
}

/// Background image struct with properties:
///  - `url` _(type: __str__)_
///  - `scale` _(type: __f32__)_
struct Image
{
	url: String,
	scale: f32,
}


/// Background enum, in which you can choose between:
/// - `Color` _(type: __str__)_
/// - `Image` _(type: __Image__)_
enum Background
{
	Color(String),
	Image(Image),
}


/// Returns the color hex value of its key (`color`).
fn parse_color(json_value: &json::JsonValue) -> Result<&str, errors::Themes>
{
	if json_value.is_empty()
	{
		return Err(errors::Themes::ColorIsEmptyError);
	}
	if ! json_value.is_string()
	{
		return Err(errors::Themes::InvalidColorTypeError);
	}
	return Ok(json_value.as_str().unwrap());
}

/// Extracts the image with its `url` and `scale` properties from json.
/// ```
/// ...
///
/// "image" : {
///     "url" : "assets/myimage"
///     "scale" : 1.0
///  }
/// ...
/// ```
fn parse_image(json_value: &json::JsonValue) -> Result<Image, errors::Themes>
{
	let bg_image_url_json = &json_value["url"];
	if bg_image_url_json.is_empty()
	{
		return Err(errors::Themes::ImageUrlIsEmptyError);
	}
	if ! bg_image_url_json.is_string()
	{
		return Err(errors::Themes::InvalidImageUrlValueError);
	}
	let bg_image_url_value = bg_image_url_json
		.as_str()
		.unwrap();

	let bg_image_scale_json = &json_value["scale"];
	let bg_image_scale_value: f32;
	if bg_image_scale_json.is_empty() || ! bg_image_scale_json.is_number()
	{
		bg_image_scale_value = 1.0;
	}
	else
	{
		bg_image_scale_value = bg_image_scale_json
			.as_f32()
			.expect(errors::Themes::InvalidImageScaleFloatValueError.msg());
	}
	return Ok(
		Image {
			url: bg_image_url_value.into(),
			scale: bg_image_scale_value,
		}
	)
}

/// Goes through all keys of background and extracts either color or image.
/// In json:
/// ```
/// ...
///
/// "background" : {
///   "image" : { // possible key
///     ...
///   }
/// }
/// ...
/// ```
fn parse_background(members: &json::iterators::Members) -> Result<Background, errors::Themes>
{
	for bg_member in members
	{
		let str_bg_member = bg_member
			.as_str()
			.unwrap();

		return match str_bg_member
		{
			"color" => Ok(
				Background::Color(
					parse_color(bg_member).unwrap()
				)
			),
			"image" => Ok(
				Background::Image(
					parse_image(bg_member).unwrap()
				)
			),
			_ => Err(errors::Themes::UnsupportedMemberError)
		}
	}
}

/// Loads a theme from a specific path.
pub fn load_theme(path: & path::PathBuf) -> Result<Theme, errors::Themes>
{
	let json_value: json::JsonValue = json::parse(
		path.join("init.json")
			.to_str()
			.unwrap()
	).expect(errors::Themes::JsonParseError.msg());

	let mut background: Background = Background::Color("#1234".into());

	// go through all keys of json_value (which is the base {} in json)
	for member in json_value.members()
	{
		let str_member = member.as_str().unwrap();
		match str_member
		{
			"background" => background = parse_background(&member.members()).unwrap(),
			_ => return Err(errors::Themes::UnsupportedMemberError),
		}
	}

	Ok(
		Theme {
			background,
		}
	)
}