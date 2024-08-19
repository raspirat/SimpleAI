use std::{
	fmt::{
		Display
	},
	error
};
use std::fmt::Formatter;
use serde::Serialize;


#[derive(Serialize, Clone, Debug)]
pub enum ErrorTypes
{
	Default,
	Critical
}

impl ErrorTypes
{
	pub fn msg(&self) -> String
	{
		match self {
			ErrorTypes::Default => String::from("Default Error"),
			ErrorTypes::Critical => String::from("Critical Error"),
		}
	}
}

/// The arguments of an error
///
/// `error_type:` either ErrorTypes::Default or ErrorTypes::Critical
/// `when:` when it happened (whilst ...)
/// `on:` what is the name of the object the error happened on
/// `message:` tells the user what to do
/// `note:` gives the user tips, further info
#[derive(Serialize, Clone, Debug)]
pub struct Args
{
	error_type: ErrorTypes,
	when: String,
	on: String,
	message: String,
	note: String
}

impl Args
{
	pub fn new(
		error_type: ErrorTypes,
		when: &str,
		on: &str,
		message: &str,
		note: &str
	) -> Self
	{
		Self {
			error_type,
			when: when.to_string(),
			on: on.to_string(),
			message: message.to_string(),
			note: note.to_string()
		}
	}

	pub fn with_on(&self, on: &str) -> Self
	{
		let mut r = self.clone();
		r.on = on.to_string();
		r
	}
}


#[derive(Serialize, Clone, Debug)]
pub enum Error
{
	FileNotFoundError(Args),
	InvalidFormatError(Args),
	NodeConversionError(Args)
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::FileNotFoundError(args) => write_error(f, args, "FileNotFoundError"),
			Error::InvalidFormatError(args) => write_error(f, args, "InvalidFormatError"),
			Error::NodeConversionError(args) => write_error(f, args, "NodeConversionError")
		}
	}
}

fn write_error(
	f: &mut Formatter<'_>,
	args: &Args,
	name: &str,
) -> std::fmt::Result
{
	write!(f, "{} ({}) whilst {} ({})\n{}\nNote:{}",
		args.error_type.msg(),
		name,
		args.when,
	  args.on,
	  args.message,
	  args.note
	)
}