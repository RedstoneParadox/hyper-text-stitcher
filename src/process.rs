use std::collections::HashMap;
use html_editor::Node;
use html_editor::Node::{Element, Text};
use html_editor::operation::{Editable, Selector};
use crate::{ComponentConfig, load_document};

// Unfortunately, the library I'm using to parse HTML doesn't like closures,
// So I'm having to do ad-hoc capture here.
static mut COMPONENTS: Option<HashMap<String, ComponentConfig>> = None;

pub fn process_page(components: &HashMap<String, ComponentConfig>, page_dom: Vec<Node>) -> Vec<Node> {
    unsafe { COMPONENTS = Some(components.clone()); }

    let page_dom = process(components, page_dom);

    unsafe { COMPONENTS = None }

    process(components, page_dom)
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
            if let Some(name) = it.name.strip_prefix("comp-") {
                let component_document = load_component_document(name);
                return component_document[0].clone()
            }
            return Element(it.clone())
        });
    }

    return processed_dom
}

fn load_component_document(name: &str) -> Vec<Node> {
    unsafe {
        let foo = COMPONENTS.clone().unwrap();
        let bar = foo.get(&name.to_string()).unwrap();
        return load_document(name, &*bar.path);
    }
}