use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PageConfig {
    pub template: String,
    pub output: String
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub include: Option<Vec<String>>,
    pub pages: HashMap<String, PageConfig>
}

pub fn load_config() -> Config {
    let config_path = Path::new(&*"hypertext-stitcher.yml");
    let config_file =  match fs::read_to_string(config_path) {
        Ok(c) => c,
        Err(e) => {
            println!("Config error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    match serde_yaml::from_str::<Config>(&*config_file) {
        Ok(c) => c,
        Err(e) =>  {
            println!("Config error(s): {}", e);
            ::std::process::exit(1);
        }
    }
}
