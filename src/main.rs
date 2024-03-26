use std::fs;
use std::path::{Path, PathBuf};
use clap::Parser;

use serde::{Deserialize, Serialize};
use tera::Context;

use config::PageConfig;
use crate::command::{Cli, Commands};
use crate::server::start_dev_server;

use crate::render::{init_tera, render_all};

mod render;
mod config;
mod command;
mod server;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Build => build_site(),
        Commands::Serve => start_dev_server()
    }
}

fn build_site() {
    let config = config::load_config();
    let tera = init_tera(&config);

    render_all(config, tera)
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
