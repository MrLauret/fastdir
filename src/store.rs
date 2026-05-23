use std::{fs::File, io::{Read, Write}, path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Dir {
    path: String,
    name: String
}

pub struct DirList{
    dirs: Vec<Dir>
}

fn get_dir() -> PathBuf {
    let homedir = dirs::home_dir().unwrap();
    let filepath = homedir.join("dirs.json");
    filepath
}

impl DirList {
    pub fn load() -> Self {
        let filepath = get_dir();

        let mut file = match File::open(filepath) {
            Ok(f) => f,
            Err(_) => return DirList { dirs: Vec::new() }
        };

        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_err() {
            return DirList { dirs: Vec::new() };
        };

        let directories: Vec<Dir> = serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
        DirList { dirs: directories }
    }

    fn save(&self) {
        let filepath = get_dir();
        
        let mut file = File::create(filepath).expect("Failed to create file");

        let json_string = serde_json::to_string_pretty(&self.dirs).expect("Failed to write file");

        file.write_all(json_string.as_bytes()).expect("Failed to save")
    }

    fn find_index(&self, name: &String) -> Result<usize, &'static str> {
        match self.dirs.iter().position(|d| &d.name == name) {
            Some(pos) => Ok(pos),
            None => Err("Not found")
        }
    }

    pub fn add(&mut self, path: &String, name: &String) -> Result<(), &'static str> {
        if self.find_index(name).is_ok() {
            return Err("Directory already has been registered");
        };

        self.dirs.push(Dir {path: path.clone(), name : name.clone()});
        self.save();
        Ok(())
    }

    pub fn delete(&mut self, name: &String) -> Result<(), &'static str> {
        let index = match self.find_index(name) {
            Ok(idx) => idx,
            Err(e) => return Err(e)
        };

        self.dirs.remove(index);
        self.save();
        Ok(())
    }

    pub fn list(&self) {
        for directorie in &self.dirs {
            println!("{}: {}", directorie.name, directorie.path)
        }
    }

    pub fn get_path(&self, name: &String) -> Result<String, &'static str> {
        match self.find_index(name) {
            Ok(idx) => Ok(self.dirs[idx].path.clone()),
            Err(e) => Err(e)
        }
    }
}