use std::error::Error;
use std::fmt::{
	Debug,
	Display,
	Formatter
};


/// Themes enum in which one can find all errors of this category.
///
/// Methods:
///
/// `msg()` -> Returns the error message  \
/// `msgf()` -> Returns a formatted error message

pub enum Themes
{
	JsonParseError,
	ImageUrlIsEmptyError,
	InvalidImageScaleFloatValueError,
	InvalidImageUrlValueError,
	UnsupportedMemberError,
	ColorIsEmptyError,
	InvalidColorTypeError,
}

impl Themes
{

	/// Method in which the error message is returned.
	pub fn msg(&self) -> String
	{
		match self
		{
			Themes::JsonParseError => String::from("[Frontend-Error]: Could not parse init.json of theme. Could be due to non existence."),
			Themes::ImageUrlIsEmptyError => String::from("[Frontend-Error]: Image key given but no value."),
			Themes::InvalidImageScaleFloatValueError => String::from("[Frontend-Error]: Unsupported scale value. Please provide a floating point value."),
			Themes::InvalidImageUrlValueError => String::from("[Frontend-Error]: Invalid url of image. Please provide a string value."),
			Themes::UnsupportedMemberError => String::from("[Frontend-Error]: Unsupported member encountered while parsing init.json of theme"),
			Themes::ColorIsEmptyError => String::from("[Frontend-Error]: Color key given but no value."),
			Themes::InvalidColorTypeError => String::from("[Frontend-Error]: Invalid color type (String required). Please provide hex colors."),
		}
	}

	/// Same method as `msg` but takes an argument that replaces `{}` in the error message (if supported).
	pub fn msgf(&self, s: &str) -> String
	{
		match self
		{
			Themes::UnsupportedMemberError => "[Internal-Alert]: Unsupported member `{}` encountered while parsing init.json of theme"
			.replace("{}", s),
			_ => self.msg().to_string(),
		}
	}

	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
	{
		f.debug_struct("Themes")
		 .field("msg", &self.msg())
		 .finish()
	}
}


impl Debug for Themes
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
	{
		return self.fmt(f);
	}
}

impl Display for Themes {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
	{
		return self.fmt(f);
	}
}

impl Error for Themes { }
