// Headifier 0.1.0
// David Serrano, October 21st, 2023
// MIT License
// Made With Love ❤️
use std::env;
use std::error::Error;
use std::fs::File;
use std::fs::{self, DirEntry};
use std::io::Write;
use std::os;
use std::path::{Path, PathBuf};

use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;



fn find(dir: &Path, name: &str) -> Option<PathBuf> {
    let mut path: Option<PathBuf> = None;
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
    
        if entry.file_name() == name {
            path = Some(entry.path()); 
            break;
        }
    }
    
    if path == None {
        return None;
    }

    Some(path.unwrap())
}

fn read_file(path: &PathBuf) -> String {
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    contents    
}

pub fn find_header(dir: &Path) -> Option<String> {
    let file_path = find(dir, "header.txt");

    match file_path {
        Some(fp) => Some(read_file(&fp)),
        None => None,
    } 
}


fn append_header(path: &PathBuf, header: &str) {
    let contents = read_file(path);

    let mut write_to_file = File::create(path).unwrap();

    write!(write_to_file, "{header}\n{contents}");
}

fn add_header_to_file(path: &PathBuf, ignore_list: &Vec<String>,
   include_list: &Vec<String>, header: &str) {
    let path_as_string = path.display().to_string();

    let mut get_ignored = false;
    for ignore in ignore_list {
        if path_as_string.contains(ignore) {
            get_ignored = true;
        }
    }

    if get_ignored {
        return;
    }

    for include in include_list.clone() {
        if path_as_string.contains(&include) {
            append_header(path, header)
        }
    }
}

fn find_get_ignore(dir: &Path) -> Result<PathBuf, String> {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();

        if entry.file_name() == ".gitignore" {
            println!("gitignore found!");
            return Ok(entry.path());
        }
    }

    Err("Could not locate .gitignore".into())
}


// return list of every file to ignore
pub fn list_git_ignore(dir: &Path) -> Vec<String> {
    let mut ignore_lines = match find_get_ignore(dir) {
        Ok(path) => {
            let mut buf_reader = BufReader::new(File::open(path).unwrap());

            let mut ignore_lines: Vec<String> = vec![];
            for line in buf_reader.lines() {
                match line {
                    Ok(l) => ignore_lines.push(l),
                    Err(e) => break,
                }
            }

            ignore_lines
        }
        Err(err) => {
            vec![]
        }
    };

    // ignore_lines.push(".toml".into());
    // ignore_lines.push(".lock".into());
    // ignore_lines.push("target".into());
    // ignore_lines.push("node_modules".into());
    // ignore_lines.push("dist".into());
    // ignore_lines.push(".json".into());
    // ignore_lines.push(".git".into());

    ignore_lines
}

pub fn visit_drs(dir: &Path, ignore_list: &Vec<String>,
    include_list: &Vec<String>, header: &str) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();

        let path = entry.path();

        if path.is_dir() {
            visit_drs(&path, &ignore_list, &include_list, header);
        } else {
            add_header_to_file(&path, &ignore_list, &include_list, header);
        }
    }
}


pub fn get_dir() -> PathBuf  {
    let path = env::current_dir();
    
    let path_buf = match path {
        Ok(p) => {
            println!("{}", p.display());
    
            p
        }
        Err(e) => {
            panic!("No current directory{e}");
        }
    };

    path_buf
}
