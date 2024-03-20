use std::collections::HashMap;
use pathdiff::diff_paths;

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
    tera.register_function("get_route_relative", get_route_relative(config.pages.clone()));

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

fn get_route_relative(page_map: HashMap<String, PageConfig>) -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value, Error> {
        let from = from_value::<String>(args.get("from").expect("oops").clone()).expect("oops");
        let to = from_value::<String>(args.get("to").expect("oops").clone()).expect("oops");

        let relative = diff_paths(&page_map.get(&*to).unwrap().output, &page_map.get(&*from).unwrap().output).expect("oops").into_os_string().into_string().expect("oops");

        return Ok(Value::from(relative))
    })
}