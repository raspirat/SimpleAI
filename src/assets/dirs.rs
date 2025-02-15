pub use std::path::PathBuf;

pub type IncludedDir = &'static include_dir::Dir<'static>;

pub mod paths {
    use super::*;

    pub fn assets() -> PathBuf {
        PathBuf::new()
    }

    pub fn themes() -> PathBuf {
        assets().join("themes")
    }

    pub fn icons() -> PathBuf {
        themes().join("icons")
    }

    pub fn styles() -> PathBuf {
        themes().join("styles")
    }

    pub fn current_icons() -> PathBuf {
        icons().join(crate::config::DEFAULT_ICONS)
    }

    pub fn current_style() -> PathBuf {
        styles().join(crate::config::DEFAULT_STYLES)
    }
}

pub fn get_dir(path: PathBuf) -> IncludedDir {
    super::include::ASSETS
        .get_dir(&path)
        .expect(format!("Couldn't get dir: {}", path.display()).as_str())
}
