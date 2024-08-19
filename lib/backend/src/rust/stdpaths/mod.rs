use std::{
	path::{
		PathBuf
	}
};

use dirs::{
	*
};

#[cfg(windows)]
mod win;

#[cfg(unix)]
mod unix;

pub static FS_NAME: &str = "sne";
pub static MAIN_CFG_FILE_NAME: &str = "init.toml";
pub static NODE_DOM_FILE_NAME: &str = "dom.json";

pub fn user_data_dir() -> PathBuf { data_local_dir().unwrap().join(FS_NAME) }
pub fn user_conf_dir() -> PathBuf { config_local_dir().unwrap().join(FS_NAME) }
pub fn app_data_dir() -> PathBuf { data_dir().unwrap().join(FS_NAME) }
pub fn app_conf_dir() -> PathBuf { config_dir().unwrap().join(FS_NAME) }

pub fn node_path() -> PathBuf { user_data_dir().join("nodes") }
pub fn theme_path() -> PathBuf { user_data_dir().join("themes") }
