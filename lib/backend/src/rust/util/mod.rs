use std::{fs::read_to_string, path::PathBuf};
use toml;

use crate::errors::{Args, Error, ErrorTypes};

use crate::stdpaths::*;

pub mod default_hash;

// #[macro_export]
// macro_rules! default_hash {
//     () => {
// 	    {
// 				let mut s = DefaultHasher::new();
// 				self.hash(&mut s);
// 				s.finish()
// 	    }
//     };
// }

macro_rules! try_from {
    ($method:expr, $path:expr, $invalid_format_error_args:expr, $file_not_found_error_args:expr) => {{
        {
            let fobj_r = read_to_string($path.join(MAIN_CFG_FILE_NAME));
            match fobj_r {
                Err(..) => Err(Error::FileNotFoundError(
                    $file_not_found_error_args.with_on($path.to_str().unwrap()),
                )),
                Ok(fobj) => {
                    let res_r = $method(fobj.as_str());
                    match res_r {
                        Err(..) => Err(Error::InvalidFormatError(
                            $invalid_format_error_args.with_on($path.to_str().unwrap()),
                        )),
                        Ok(res) => Ok(res),
                    }
                }
            }
        }
    }};
}

// pub fn try_from<'a, T, E: std::error::Error>(
// 	method: fn(&'a str) -> Result<T, E>,
// 	path: PathBuf,
// 	invalid_format_error_args: Args,
// 	file_not_found_error_args: Args
// ) -> Result<T, Error>
// {
//
// }

pub fn try_from_toml<T>(path: PathBuf) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    try_from! {
        toml::from_str::<T>,
        path,
        Args::new(
            ErrorTypes::Default,
            "Trying to serialize toml",
            "",
            "Could not serialize from init.toml",
            "Your format is probably invalid. (check your syntax)"
        ),
        Args::new(
            ErrorTypes::Default,
            "Trying to read init.toml",
            "",
            "Could find or open init.toml",
            "Your plugin is probably invalid. (no init.toml present in folder)"
        )
    }
}

pub fn try_from_json<'de, T>(path: PathBuf) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    try_from! {
        serde_json::from_str::<T>,
        path,
        Args::new(
            ErrorTypes::Default,
            "Trying to serialize json",
            "",
            "Could not serialize from .json",
            "Your format is probably invalid. (check your syntax)"
        ),
        Args::new(
            ErrorTypes::Default,
            "Trying to read json",
            "",
            "Could find or open .json",
            "Your plugin is probably invalid. (no dom.json present in folder)"
        )
    }
}
