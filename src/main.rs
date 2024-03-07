use std::collections::HashMap;
use std::fs;
use std::path::Path;
use html_editor::operation::Htmlifiable;
use html_editor::parse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub components: HashMap<String, ComponentConfig>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ComponentConfig {
    pub path: String,
    pub route: Option<String>
}

fn main() {
    println!("Hello, world!");

    let config_file = fs::read_to_string("components.yml").unwrap();
    let config: Config = serde_yaml::from_str(&*config_file).unwrap();

    for pair in config.components {
        let name = pair.0;
        let component_config = pair.1;
        let document_path_string = format!("components{}/{}.html", component_config.path, name);
        let document_path = Path::new(&*document_path_string);
        let document_contents = fs::read_to_string(document_path).unwrap();
        let mut dom = parse(&*document_contents).unwrap();

        // Do script processing here

        if let Some(route) = component_config.route {
            let html = dom.html();
            let page_path_string = format!("html{}/index.html", route);
            let page_path = Path::new(&*page_path_string);
            fs::write(page_path, &*html).unwrap();
        }
    }
}
