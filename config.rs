use std::env;
use config::{Config, File};

impl Configuration {
    pub fn new(){
        let env = env::var("ENV").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            .add_source(File::)
    }
}