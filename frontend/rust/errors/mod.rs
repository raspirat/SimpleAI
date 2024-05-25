pub struct UneError<'a>
{
	message: &'a str
}

impl UneError<'_>
{
	pub const fn as_str(&self) -> &str
	{
		&self.message
	}

	pub fn print_message(&self)
	{
		println!("{}", self.message);
	}
}



// impl From<&str> for UneError<'_>
// {
// 	fn from(value: &str) -> Self
// 	{
// 		Self {
// 			message: value
// 		}
// 	}
// }

// impl From<&str> for UneError
// {
// 	fn from(value: &str) -> Self {
// 		Self {
// 			message: value.to_string()
// 		}
// 	}
// }
//
// impl From<String> for UneError
// {
// 	fn from(value: String) -> Self {
// 		Self {
// 			message: value
// 		}
// 	}
// }

pub trait FormatMessage
{
	fn print_message(&self, replacement: &str);
}


impl FormatMessage for UneError<'_>
{
	fn print_message(&self, replacement: &str)
	{
		println!("{}", self.message.replace("{}", replacement));
	}
}


use std::error::Error;
use std::fmt::{
	Debug,
	Display,
	Formatter
};

impl Debug for UneError<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("UNEError")
			.field("message", &self.message)
			.finish()
	}
}

impl Display for UneError<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("UNEError")
		 .field("message", &self.message)
		 .finish()
	}
}

impl Error for UneError<'_> {}

pub mod themes
{
		use crate::errors::{FormatMessage, UneError};

		pub enum Errors
		{
			PARSE_ERROR
			IMAGE_URL_NOT_FOUND_ERROR
			I
		}
		pub static  PARSE_ALERT_MESSAGE: UneError = UneError {
			message: "[Internal-Alert]: Could not parse init.json of theme. Could be due to non existence."
		};
		pub static IMAGE_NOT_FOUND_URL_MESSAGE: UneError = UneError {
			message: "[Internal-Alert]: Could not find url of image."
		};
		pub static IMAGE_SCALE_FLOAT_VALUE_MESSAGE: UneError = UneError {
			message: "[Internal-Alert]: Unsupported scale value. Please provide a floating point value."
		};
		pub static UNSUPPORTED_MEMBER_MESSAGE: UneError = UneError {
			message: "[Internal-Alert]: Unsupported member {} encountered while parsing init.json of theme"
		};
		pub static THEME_ERROR: UneError = UneError {
			message: "Unable to load selected theme."
		};
		pub static IMAGE_INVALID_URL_MESSAGE: UneError = UneError {
			message: "[Internal-Alert]: Invalid url of image. Please provide a string value."
		};
}