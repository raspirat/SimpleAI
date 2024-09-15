use std::{fs::read_to_string, path::PathBuf, str::FromStr};

use libbacksne::info::Info;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
struct Theme {
    info: Info,
}

impl Theme {
    pub fn new(info: Info) -> Self {
        Self { info }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new(Info::new(
            "DefaultTheme",
            "1.0.0",
            "This is the default theme.",
            "sert",
            vec!["default, theme"],
            Url::from_str("https://github.com/sertschgi").unwrap(),
            vec!["themes"],
        ))
    }
}

impl From<PathBuf> for Theme {
    fn from(path: PathBuf) -> Self {
        let fobj = read_to_string(path);
        if fobj.is_ok() {
            let sfobj = fobj.unwrap();
            return toml::from_str(sfobj.as_str()).unwrap_or(Self::default());
        }
        Self::default()
    }
}
