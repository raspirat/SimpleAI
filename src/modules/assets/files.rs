pub use std::path::PathBuf;

pub type IncludedFile = &'static include_dir::File<'static>;

pub fn get_file(path: PathBuf) -> IncludedFile {
    super::include::ASSETS
        .get_file(&path)
        .expect(format!("Couldn't get file: {}", path.display()).as_str())
}
