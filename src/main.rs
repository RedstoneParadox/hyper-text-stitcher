mod template;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use tera::{Context};
use crate::template::init_terra;


#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub include: Option<Vec<String>>,
    pub pages: HashMap<String, PageConfig>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PageConfig {
    pub template: String,
    pub output: String
}

fn main() {
    let config = load_config();
    let tera = init_terra(&config);

    for pair in config.pages {
        let name = pair.0;
        let page = pair.1;
        let mut context = Context::new();

        context.insert("page", &name);
        println!("Rendering \"{}\"", page.output);

        let rendered = match tera.render(&*page.template, &context) {
            Ok(p) => p,
            Err(e) => {
                println!("Rendering error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        save_rendered_page(page, &*rendered)
    }

    if let Some(dirs) = config.include {
        for from in dirs {
            println!("Copying directory \"{}\" to output directory", from);
            let to = format!("html/{}", from);
            copy(&Path::new(&*from), &Path::new(&*to)).expect("TODO: panic message");
        }
    }
}

fn load_config() -> Config {
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

fn save_rendered_page(page: PageConfig, rendered: &str) {
    let mut split: Vec<&str> = page.output.split("/").collect();
    split.pop();
    let foo = split.join("/");

    let output_dir = format!("html/{}", foo);
    let output_dir_path = Path::new(&*output_dir);

    let output_location = format!("html/{}", page.output);
    let output_location_path = Path::new(&*output_location);

    fs::create_dir_all(&output_dir_path).expect("TODO: panic message");
    fs::write(&output_location_path, rendered).expect("TODO: panic message");
}

pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                    }
                }
            }
        }
    }

    Ok(())
}
