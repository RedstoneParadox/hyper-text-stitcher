mod process;

use std::collections::HashMap;
use std::fs;
use std::path::{Components, Path};
use html_editor::operation::{Editable, Htmlifiable, Queryable, Selector};
use html_editor::{Node, parse};
use html_editor::Node::{Element, Text};
use serde::{Deserialize, Serialize};
use crate::process::process_page;


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
    println!("Hello, world!");

    let config_file = fs::read_to_string("components.yml").unwrap();
    let config: Config = serde_yaml::from_str(&*config_file).unwrap();

    for pair in config.pages {
        let name = pair.0;
        let page_config = pair.1;
        let mut dom = load_document(&*name, &*page_config.path);

        dom = process_page(&config.components, dom);

        let html = dom.html();
        let page_path_string = format!("html{}/index.html", page_config.route);
        let page_path = Path::new(&*page_path_string);
        fs::write(page_path, &*html).unwrap();
    }
}

fn process(components: &HashMap<String, ComponentConfig>, dom: Vec<Node>) -> Vec<Node> {
    let mut processed_dom = dom;

    for component in components {
        let name = component.0;
        let element_name = format!("comp-{}", name);
        let config = component.1;
        let component_document = load_document(&*name, &*config.path);
        let selector = Selector::from(&*element_name);
        let processed_dom = processed_dom.replace_with(&selector, |it| {
            Text("hello".to_string())
        });
    }

    return processed_dom
}

fn load_document(name: &str, path: &str) -> Vec<Node> {
    let document_path_string = format!("components{}/{}.html", path, name);
    let document_path = Path::new(&*document_path_string);
    let document_contents = fs::read_to_string(document_path).unwrap();
    return parse(&*document_contents).unwrap();
}
