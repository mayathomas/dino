use anyhow::Result;
use git2::Repository;
use std::{fs, path::Path};

use askama::Template;
use clap::Parser;
use dialoguer::Input;

use crate::CmdExecutor;

#[derive(Debug, Parser)]
pub struct InitOpts {}

#[derive(Template)]
#[template(path = "config.yml.j2")]
#[allow(unused)]
struct ConfigFile {
    name: String,
}

#[derive(Template)]
#[template(path = "main.ts.j2")]
struct MainTsFile {}

#[derive(Template)]
#[template(path = ".gitignore.j2")]
struct GitIgnoreFile {}

impl CmdExecutor for InitOpts {
    async fn execute(self) -> Result<()> {
        let name: String = Input::new().with_prompt("Project name").interact_text()?;
        // if current is empty then init project, otherwise create new dir
        let cur = Path::new(".");
        if fs::read_dir(".")?.next().is_none() {
            init_project(&name, cur)
        } else {
            let path = cur.join(&name);
            init_project(&name, &path)
        }
    }
}

fn init_project(name: &str, path: &Path) -> Result<()> {
    Repository::init(path)?;
    let config = ConfigFile {
        name: name.to_string(),
    };
    fs::write(path.join("config.yml"), config.render()?)?;
    fs::write(path.join("main.ts"), MainTsFile {}.render()?)?;
    fs::write(path.join(".gitignore"), GitIgnoreFile {}.render()?)?;
    Ok(())
}
