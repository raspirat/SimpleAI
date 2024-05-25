use std::path::*;

use json;

use crate::errors::UneError;
use crate::errors::FormatMessage;
use crate::errors::themes::*;

// enum ThemeBackground
// {
// 	Color(String),
// 	Image {
// 		url: String,
// 		scale: f32
// 	}
// }
//
// impl From<&str> for ThemeBackground
// {
// 	fn from(value: &str) -> Self {
// 		Self::Color(value.into())
// 	}
// }
//
// impl Default for ThemeBackground
// {
// 	fn default() -> Self {
// 		ThemeBackground::Color("#1234".into())
// 	}
// }
//
// pub struct Theme
// {
// 	background: ThemeBackground
// }

pub fn get_theme(path: & PathBuf) // -> Result<Theme, UneError>
{
	let json_value: json::JsonValue = json::parse(
		path.join("init.json")
			.to_str()
			.unwrap()
	).expect(PARSE_ALERT_MESSAGE.as_str());
	PARSE_ALERT_MESSAGE.print_message();

	// let mut background = ThemeBackground::default();
	// slint::Global::get()
	

	for member in json_value.members()
	{
		let str_member = member.as_str().unwrap();
		match str_member
		{
			"background" => {
				for bg_member in member.members()
				{
					let members = bg_member.members();
					let str_bg_member = bg_member.as_str().unwrap();
					match str_bg_member
					{
						// "color" => background = members.last()
						// 	.unwrap()
						// 	.as_str()
						// 	.unwrap()
						// 	.into(),
						"image" => {
							let bg_image_json = &bg_member["image"];

							let bg_image_url_json = &bg_image_json["url"];

							if bg_image_url_json[0].is_empty()
							{
								&IMAGE_NOT_FOUND_URL_MESSAGE;
							}

							if ! bg_image_url_json[0].is_string()
							{
								&IMAGE_INVALID_URL_MESSAGE;
							}

							let bg_image_url_value = bg_image_url_json[0]
								.as_str()
								.unwrap();

							let bg_image_scale_json = &bg_image_json["scale"];
							let bg_image_scale_value: f32;
							if bg_image_scale_json.is_empty() || ! bg_image_scale_json[0].is_number()
							{
								bg_image_scale_value = 1.0;
							}
							else
							{
								bg_image_scale_value = bg_image_scale_json[0]
									.as_f32()
									.expect(IMAGE_SCALE_FLOAT_VALUE_MESSAGE.as_str());
							}
							// background = ThemeBackground::Image {
							// 	url: bg_image_url_value.into(),
							// 	scale: bg_image_scale_value
							// };
						}
						_ => FormatMessage::print_message(&UNSUPPORTED_MEMBER_MESSAGE, str_bg_member)
					}
				}
			}
			_ => FormatMessage::print_message(&UNSUPPORTED_MEMBER_MESSAGE, str_member)
		}
	}

	// Ok(
	// 	Theme {
	// 		background
	// 	}
	// )
}