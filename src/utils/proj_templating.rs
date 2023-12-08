use std::fs;

use git2::{Error, Repository};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct ProjTemplating {
    repo: Repository,
}

impl ProjTemplating {
    /// Returns a list of all available templates from source repos
    ///
    /// # Returns
    ///
    /// * `Result<Vec<String>, Error>` - a list of templates or a libgit2 error.
    pub fn list_templates() {}

    /// Returns a list of template source repos
    ///
    pub fn list_template_repos() {}

    pub fn add_template_repo(url: String) {
        let home = format!("{}/.cc_template_repos", std::env::var_os("HOME").expect("Unable to read $HOME env var!").to_str().unwrap());
        if let Ok(dir) = fs::try_exists(&home) {
            if !dir {
                fs::create_dir(&home).expect("Unable to create template repositories directory!");
            }
        }
        if let Ok(repo) = Repository::clone(&(*url), &home) {
            let repo_dir = repo.path().parent();
            let manifest = fs::read_to_string(format!("{:?}/manifest.json", repo_dir)).expect("Unable to read Manifest.json!");
            let mut res: TemplateRepositories = serde_json::from_str(&(*manifest)).expect("msg");
            println!("{:?}", res.repos);
        }
    }

    pub fn parse_repo_name(url: String) -> String {
        // let mut split: Vec<&str> = url.split("/").collect();
        // let repo_name = split.pop().unwrap().split(".").
        // String::from(split.pop().unwrap())
        let re = Regex::new(r"[a-zA-Z0-9-_]*(\.git)gm");
    }

    pub fn refresh_templates(&self) {}

    pub fn clone_template(url: String, path: String) -> Result<Self, Error> {
        let repo = Repository::clone(&(*url), path)?;
        Ok(Self { repo })
    }

    pub fn prune_commits(&self) {}

    pub fn initial_commit(&self) {}
}

pub struct Template {
    name: String,
    description: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateRepositories {
    pub repos: Vec<TemplateRepo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateRepo {
    pub org: String,
    pub name: String,
    pub description: String,
    pub url: String,
}
