use std::env;
use std::error::Error;
use std::fs::File;
use std::fs::{self, DirEntry};
use std::io::Write;
use std::os;
use std::path::{Path, PathBuf};

use std::io::BufReader;
use std::io::Read;

fn append_header(path: &PathBuf, header: &str) {
    let file = File::open(path).unwrap();

    println!("Adding header: {:?}", path);

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let mut write_to_file = File::create(path).unwrap();

    write!(write_to_file, "{header}\n{contents}");
}

fn add_header_to_file(path: &PathBuf, ignore_list: &Vec<String>, header: &str) {
    let includes = files_to_add();
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

    for include in includes {
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

fn files_to_add() -> Vec<String> {
    vec![".js".into(), ".ts".into(), ".rs".into(), ".go".into()]
}

// return list of every file to ignore
fn list_git_ignore(dir: &Path) -> Vec<String> {
    find_get_ignore(dir);
    
    vec![
        ".toml".into(),
        ".lock".into(),
        "target".into(),
        "node_modules".into(),
        "dist".into(),
        ".json".into(),
        ".git".into(),
        ]
    }
    
    fn visit_drs(dir: &Path, ignore_list: &Vec<String>, header: &str) {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
    
            let path = entry.path();
    
            if path.is_dir() {
                visit_drs(&path, &ignore_list, header);
            } else {
                add_header_to_file(&path, &ignore_list, header);
            }
        }
    }

pub fn write_all_headers(header: &str) {
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

    let ignore_list = list_git_ignore(&path_buf);
    visit_drs(&path_buf, &ignore_list, &header);
}