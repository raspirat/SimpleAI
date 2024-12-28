use std::path::PathBuf;
use copy_dir;

fn copy_assets()
{
  let dir = PathBuf::from("target/debug/assets");
	if dir.exists() { std::fs::remove_dir_all(&dir).unwrap(); }
	copy_dir::copy_dir(
		"../assets",
		dir
	).unwrap();
}

fn main() {
	copy_assets()
}