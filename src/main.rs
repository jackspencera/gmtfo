use std::env;
use std::fs;
use std::fs::DirEntry;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    targets: Vec<String>,
}

impl ::std::default::Default for Config {
    fn default() -> Self { Self { targets: vec![".git".to_string(), ".config".to_string()] } }
}

fn main() -> Result<(), confy::ConfyError> {
    let cfg: Config = match confy::load("gmtfo") {
        Ok(config) => config,
        Err(_) => Config::default()
    };
    confy::store("gmtfo", &cfg).unwrap();
    let curr_dir = env::current_dir().unwrap();
    match curr_dir.to_str() {
        Some(dir_str) => {
            let dir_tokens = dir_str.split('/').collect();
            process_dir_tokens(dir_tokens, &cfg.targets).unwrap();
        }
        None => panic!("Unable to parse directory")
    }
    Ok(())
}

fn process_dir_tokens(dir_tokens: Vec<&str>, targets: &Vec<String>) -> Result<(), std::io::Error> {
    for i in (2..dir_tokens.len()).rev() {
        let sub_dir = &dir_tokens[..i];
        let full_path = sub_dir.join("/");
        if check_dir(&full_path, targets) {
            println!("{}", full_path);
            return Ok(());
        }
    }
    Ok(())
}

fn check_dir(directory: &String, targets: &Vec<String>) -> bool {
    let mut folders = fs::read_dir(&directory).unwrap();
    folders.find_map(|path| check_path(path.unwrap(), targets)).is_some()
}

fn check_path(dir_entry: DirEntry, targets: &Vec<String>) -> Option<()> {
    let path = dir_entry.path();
    for target in targets {
        let curr_path = path.to_str().unwrap();
        if curr_path.ends_with(target) {
            return Some(())
        }
    }
    None
}
