

use std::collections::HashMap;
use std::fs;
use std::path::{Path};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};


#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub components: HashMap<String, ComponentConfig>,
    pub pages: HashMap<String, PageConfig>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ComponentConfig {
    pub path: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PageConfig {
    pub path: String,
    pub route: String
}



fn main() {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let page = match tera.render("home.html", &Context::new()) {
        Ok(p) => p,
        Err(e) => {
            println!("Rendering error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    
    fs::write(&Path::new(&*"html/index.html"), &*page).expect("TODO: panic message");
}
