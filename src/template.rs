use std::collections::HashMap;

use tera::{Error, from_value, Function, Tera, to_value, Value};

use crate::{Config, PageConfig};

pub fn init_terra(config: &Config) -> Tera {
    let mut tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    tera.register_function("get_route", get_route(config.pages.clone()));

    return tera;
}

fn get_route(page_map: HashMap<String, PageConfig>) -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value, Error> {
        match args.get("page") {
            None => Err("oops".into()),
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => {
                    let page = page_map.get(&v).unwrap();
                    Ok(to_value(&page.output).unwrap())
                }
                Err(_) => Err("oops".into()),
            }
        }
    })
}