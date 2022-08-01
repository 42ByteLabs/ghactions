
use std::path::Path;
use dotenv::dotenv;
use std::{env, collections::HashMap};
use log::{info, debug, warn};

use crate::models::ActionYML;



fn load_environment_variables(prefix: &str) -> HashMap<String, String> {
    let mut list = HashMap::<String, String>::new();
    for (env_key, env_value) in env::vars() {
        if env_key.starts_with(&prefix) {
            let fkey = format!("{}_", prefix);
            let new_key = env_key.replace(&fkey, "").to_lowercase();
            debug!("Loading `{}` Env Key: {}", prefix, new_key);
            list.insert(new_key, env_value);
        }
    }
    list
}


#[derive(Debug)]
pub struct GHAction {
    // Path to the root of the Action code
    pub path: String,
    // `action.yml` path
    pub action_file_path: String, 

    pub name: Option<String>,
    pub description: Option<String>,

    pub inputs: HashMap<String, String>,

    // https://docs.github.com/en/actions/learn-github-actions/environment-variables
    pub github: HashMap<String, String>,
    pub runner: HashMap<String, String>,

    pub loaded: bool,
}

impl GHAction {
    pub fn new() -> Self {
        debug!("Loading dotenv...");
        dotenv().ok();

        let action_path: String = env::var("GITHUB_ACTION_PATH").unwrap_or_else(|_| "./".to_string());
        
        GHAction {
            path: action_path,
            action_file_path: "action.yml".to_string(),
            name: None,
            description: None,
            inputs: load_environment_variables("INPUT"),
            github: load_environment_variables("GITHUB"),
            runner: load_environment_variables("RUNNER"),
            loaded: false
        }
    }

    pub fn in_action(&mut self) -> bool {
        Path::new(&self.action_path()).exists()
    }

    pub fn set_path(&mut self, path: String) -> &mut Self {
        self.action_file_path = Path::new(&path).join("action.yml")
            .to_str().unwrap_or("./action.yml").to_string();
        self.path = path;
        self
    }

    fn action_path(&mut self) -> String {
        let action_file_path: String = Path::new(&self.path).join(&self.action_file_path)
            .to_str().unwrap_or("./action.yml").to_string();
        
        action_file_path
    }

    fn encode_envvar(prefix: &str, key: &String) -> String {
        let new_key: String = key.clone().replace('-', "_").to_uppercase();
        format!("{}_{}", prefix, &new_key)
    }
    
    pub fn get(&mut self, key: &str) -> Option<String> {
        let new_key = key.to_lowercase();
        if self.github.contains_key(&new_key) {
            return Some(self.github.get(&new_key).unwrap().to_string());
        }
        if self.inputs.contains_key(&new_key) {
            return Some(self.inputs.get(&new_key).unwrap().to_string());
        }

        None
    } 

    pub fn get_input(&mut self, input: &str) -> Option<String> {
        let new_input = input.to_lowercase();
        if self.inputs.contains_key(&new_input) {
            debug!("Action input found: {}", &input);
            return Some(self.inputs.get(&new_input).unwrap().to_string());
        }
        None
    }

    pub fn load_actions_file(&mut self) -> &mut Self {
        let action_file_path = self.action_path();
        info!("Loading Action file: {}", &action_file_path);

        match ActionYML::load_action(action_file_path) {
            Ok(action_yml) => {
                info!("Found and loaded Actions YML file"); 

                self.name = action_yml.name;
                self.description = action_yml.description;

                for (key, _value) in action_yml.inputs.into_iter() {
                    let key_envvar = GHAction::encode_envvar("INPUT", &key);
                    match env::var(key_envvar) {
                        Ok(v) => {
                            debug!("Found key and envvar: {}", &key);
                            self.inputs.insert(key, v);
                        },
                        Err(_e) => { 
                            warn!("Failed to find key: {}", &key);
                        }
                    }
                }
                self.loaded = true;
            },
            Err(e) => {
                warn!("Failed to load inputs: {}", e);
            },
        };

        self
    }
}



