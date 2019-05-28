use config::{Config, ConfigError, File};
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Settings {
    debug: bool,
    test: String
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let mut c = Config::new();

        c.merge(File::with_name("cfg"));

        c.try_into()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self{debug: true, test: String::new()}
    }
}


//pub
