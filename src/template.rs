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
        let relative= computer_relative_path(page_map.get(&*from).unwrap().clone().output, page_map.get(&*to).unwrap().clone().output);

        return Ok(Value::from(relative))
    })
}

fn computer_relative_path(from: String, to: String) -> String {
    // If they're the same path, just save the computation path
    if from == to {
        return "".to_string();
    }

    let mut from_split: Vec<&str> = from.split("\\").collect();
    let mut to_split: Vec<&str> = to.split("\\").collect();

    // Reverse the paths so that we're looking at them from the bottom up
    from_split.reverse();
    to_split.reverse();

    // Eliminate the common root of each path
    while from_split.last().unwrap() == to_split.last().unwrap() {
        from_split.pop();
        to_split.pop();
    }

    let from_len = from_split.len();
    let to_len = to_split.len();

    // Both "to" and "from" point to a file in the same directory
    // or else "to" is in a subdirectory of "from"'s directory
    if from_len == 1 && to_len >= 1 {
        return to_split.pop().unwrap().to_string();
    }


    let mut rel_path: Vec<String> = vec![];

    while from_split.len() > 1 {
        from_split.pop();
        rel_path.push("..".to_string())
    }

    while !to_split.is_empty() {
        rel_path.push(to_split.pop().unwrap().to_string())
    }

    return rel_path.join("\\")
}