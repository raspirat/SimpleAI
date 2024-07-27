use std::{
	path::{
		PathBuf
	}
};

use dirs::{
	*
};

static FS_NAME: String = "sne".into();
static HIDDEN_FS_NAME: String = ".".into() + FS_NAME;
pub static USER_DATA_DIR: PathBuf = home_dir().unwrap().join(HIDDEN_FS_NAME);
pub static APP_DATA_DIR: PathBuf = data_dir().unwrap().join(FS_NAME);

