// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::hash::{DefaultHasher, Hash, Hasher};

use serde::Serialize;
use walkdir::WalkDir;

#[derive(Serialize)]
struct ProjectStructure {
    name: String,
    root_path: String,
    root_hash: String,
    files: Vec<ProjectEntry>,
}

impl From<String> for ProjectStructure {
    fn from(path: String) -> Self {
        let name = path.split('\\').last().unwrap().to_string();
        let root_path = path.clone();
        ProjectStructure {
            name,
            root_path,
            files: Vec::new(),
            root_hash: String::new(),
        }
    }
}

impl From<WalkDir> for ProjectStructure {
    fn from(walker: WalkDir) -> Self {
        let mut project_iterator = walker.into_iter().filter_map(std::result::Result::ok);

        match project_iterator.next() {
            Some(root_entry) => {
                let mut files = Vec::new();

                for entry in project_iterator {
                    let project_entry = ProjectEntry::from_walkdir(&entry);
                    files.push(project_entry);
                }

                ProjectStructure {
                    name: root_entry
                        .path()
                        .to_str()
                        .unwrap()
                        .split('\\')
                        .last()
                        .unwrap()
                        .to_string(),
                    root_path: root_entry.path().to_str().unwrap().to_string(),
                    root_hash: hash_path(root_entry.path().to_str().unwrap().to_string()),
                    files,
                }
            }
            None => ProjectStructure {
                name: String::new(),
                root_path: String::new(),
                root_hash: String::new(),
                files: Vec::new(),
            },
        }
    }
}

#[derive(Serialize, Hash)]
struct ProjectEntry {
    name: String,
    path: String,
    hash: String,
    is_file: bool,
    parent_hash: String,
}

impl ProjectEntry {
    fn from_walkdir(entry: &walkdir::DirEntry) -> Self {
        let root_path = entry.path().to_str().unwrap().to_string();
        let name = root_path.split('\\').last().unwrap().to_string();
        let hash = hash_path(root_path.clone());
        let parent_hash = hash_path(entry.path().parent().unwrap().to_str().unwrap().to_string());

        ProjectEntry {
            name,
            path: root_path,
            hash,
            is_file: entry.path().is_file(),
            parent_hash,
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Walkdir error: {0}")]
    WalkDir(#[from] walkdir::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

type Result<T> = std::result::Result<T, Error>;

fn hash_path(path: String) -> String {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    hasher.finish().to_string()
}

#[tauri::command]
async fn get_project_structure(mut path: String) -> Result<ProjectStructure> {
    path = path.replace('/', "\\");
    let walker = walkdir::WalkDir::new(path).max_depth(1);
    println!("project structure started...");
    let ps = ProjectStructure::from(walker);
    println!("project structure finished");
    Ok(ps)
}

#[tauri::command]
fn read_file(path: String) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_project_structure, read_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
