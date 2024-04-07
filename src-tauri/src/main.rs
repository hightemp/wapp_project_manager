// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::io::Write;
use std::fs::File;
use tauri::api::path::home_dir;
use std::sync::Mutex;
use std::path::PathBuf;
use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Settings {
    projects_paths: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Project {
    title: String,
    name: String,
    short_description: String,
    path: String,
    markdown: String,
    has_readme: bool,
}

type ProjectsList = Vec<Project>;

const SETTING_FILE_NAME: &str = "wapp_project_manager.settings.json";
lazy_static! {
    static ref SETTINGS: Mutex<Settings> = Mutex::new(Settings {
        projects_paths: Vec::new(),
    });
    static ref PROJECTS: Mutex<ProjectsList> = Mutex::new(Vec::new());
}

fn load_settings() -> Settings {
    let home_dir = home_dir().expect("Failed to get home directory");
    let settings_path = home_dir.join(SETTING_FILE_NAME);

    if !settings_path.exists() {
        let mut file = File::create(&settings_path).expect("Failed to create settings file");
        let json = serde_json::to_string_pretty(&(*SETTINGS.lock().unwrap())).expect("Failed to serialize settings");
        file.write_all(json.as_bytes()).expect("Failed to write settings file");
    }

    let file = File::open(settings_path).expect("Failed to open settings file");
    serde_json::from_reader(file).expect("Failed to deserialize settings")
}

fn scan_readme_file(file_path: PathBuf, project_path: &PathBuf) -> Result<Project, String> {
    let file_content = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Error reading file: {}", e)),
    };

    let re = Regex::new(r#"# ([^\n]+)([^#]+?)"#).unwrap();

    if let Some(captures) = re.captures(&file_content) {
        let title = captures.get(1).unwrap().as_str().to_string();
        let short_description = captures.get(2).unwrap().as_str().trim().to_string();
        
        let project = Project {
            title,
            name: String::from(project_path.file_name().unwrap().to_str().unwrap()),
            short_description,
            path: String::from(project_path.to_str().unwrap()),
            markdown: file_content,
            has_readme: true,
        };

        Ok(project)
    } else {
        Err("No title and description found in README file".to_string())
    }
}

fn scan_dir_for_readme(input_path: PathBuf) -> Result<Project, String> {
    let project_entries = fs::read_dir(&input_path).unwrap();
    let readme_re = Regex::new(r#"(?i)^readme(?:\.md|\.txt|\.markdown)?$"#).unwrap();

    for project_file_entry in project_entries {
        let project_file_path = project_file_entry.unwrap().path();

        if project_file_path.is_file() {
            let file_name = project_file_path.file_name().unwrap().to_str().unwrap();

            if readme_re.is_match(file_name) {
                let project_result = scan_readme_file(project_file_path, &input_path);
                if project_result.is_ok() {
                    let project = project_result.unwrap();
                    return Ok(project)
                }
            }
        }
    }

    let input_path_name = String::from((&input_path).file_name().unwrap().to_str().unwrap());
    return Ok(Project { 
        title: input_path_name.clone(), 
        name: input_path_name.clone(),
        short_description: String::from(""), 
        path: String::from(input_path.to_str().unwrap()),
        markdown: String::from(""), 
        has_readme: false 
    })
}

fn scan_projects_dirs() -> ProjectsList {
    let settings: Settings = SETTINGS.lock().unwrap().clone();
    let mut projects_list: ProjectsList = vec![];

    for projects_path in settings.projects_paths {
        let projects_path_buf = PathBuf::from(projects_path);
        
        let projects_entries = fs::read_dir(projects_path_buf).unwrap();

        for entry_result in projects_entries {
            let entry_path = entry_result.unwrap().path();

            if entry_path.is_dir() {
                let scan_result = scan_dir_for_readme(entry_path);
                if scan_result.is_ok() {
                    projects_list.push(scan_result.unwrap())
                }
            }
        }
    }

    return projects_list
}

#[tauri::command]
fn get_settings() -> Result<Settings, String> {
    Ok(SETTINGS.lock().unwrap().clone())
}

#[tauri::command]
fn update_projects_list() -> Result<(), String> {
    *PROJECTS.lock().unwrap() = scan_projects_dirs();
    Ok(())
}

#[tauri::command]
fn get_projects_list() -> Result<ProjectsList, String> {
    Ok(PROJECTS.lock().unwrap().clone())
}

fn main() {
    *SETTINGS.lock().unwrap() = load_settings();

    update_projects_list().expect("scan projects error");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_settings, 
            update_projects_list,
            get_projects_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
