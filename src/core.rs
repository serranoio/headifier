// Headifier 0.1.0
// David Serrano, October 21st, 2023
// MIT License
// Made With Love ❤️
use std::env;
use std::fs::File;
use std::fs::{self};
use std::io::{Write, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

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


fn prepend_header(path: &PathBuf, header: &str) {
    let contents = read_file(path);

    let mut write_to_file = File::create(path).unwrap();

    write!(write_to_file, "{header}\n{contents}").unwrap();
}

fn add_header_to_file(path: &PathBuf, ignore_list: &Vec<String>,
   include_list: &Vec<String>, header: &str,
    applied_list:  &mut Vec<String>) {
    let path_as_string = path.display().to_string().to_ascii_lowercase();


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
            applied_list.push(path.display().to_string());
            prepend_header(path, header)
        }
    }
}

fn find_get_ignore(dir: &Path) -> Result<PathBuf, String> {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();

        if entry.file_name() == ".gitignore" {
            return Ok(entry.path());
        }
    }

    Err("Could not locate .gitignore".into())
}


// return list of every file to ignore
pub fn list_git_ignore(dir: &Path) -> Vec<String> {
    let ignore_lines = match find_get_ignore(dir) {
        Ok(path) => {
            let buf_reader = BufReader::new(File::open(path).unwrap());

            let mut ignore_lines: Vec<String> = vec![];
            for line in buf_reader.lines() {
                match line {
                    Ok(l) => {
                        if !(l.len() == 0) {
                            ignore_lines.push(l.to_ascii_lowercase())
                        }
                    } Err(_) => break,
                }
            }

            ignore_lines
        }
        Err(_) => {
            vec![]
        }
    };

    ignore_lines
}

pub fn visit_drs(dir: &Path, ignore_list: &Vec<String>,
    include_list: &Vec<String>, header: &str,
     applied_list:  &mut Vec<String>) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();

        let path = entry.path();

        if path.is_dir() {
            visit_drs(&path, &ignore_list, &include_list, header, applied_list);
        } else {
            add_header_to_file(&path, &ignore_list, &include_list, header, applied_list);
        }
    }
}


pub fn get_dir() -> PathBuf  {
    let path = env::current_dir();
    
    let path_buf = match path {
        Ok(p) => {
            p
        }
        Err(e) => {
            panic!("{e}");
        }
    };

    path_buf
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::Path;
    use std::path::PathBuf;

    use super::get_dir;
    use super::find_get_ignore;
    use super::read_file;
    use super::visit_drs;
    use std::io::Write;

    #[test]
    pub fn test_get_dir() {
        let path = get_dir();
        let path = path.display();
        
        assert_eq!(env::current_dir().unwrap().display().to_string(), path.to_string());
    }
    
    #[test]
    pub fn test_find_get_ignore() {
        let dir = get_dir();

        match find_get_ignore(&dir) {
            Ok(found) => {
                let found = found.display();
                let found = found.to_string();
                assert_eq!("/Users/davidserrano/greatness/rust/headifier/.gitignore", found);

            } Err(not_found) => {
                assert_eq!(not_found, "Could not locate .gitignore")
            }
        }
    }

    fn create_file(test_file: &PathBuf) -> String {
        let mut handle = std::fs::File::create(test_file).unwrap();

        let contents = "I want to stay the same!"; 

        write!(handle, "{contents}").unwrap();

        contents.into()
    }

    #[test]
    pub fn test_visit_dirs() {
        // contents before
        let main_file_path = PathBuf::from("src/main.rs");
        let contents_before = read_file(&main_file_path);
        
        let ignore_list: Vec<String> = vec!["/target", "cargo.toml", "cargo.lock", "README.md"].into_iter().map(|s| s.to_string()).collect();
        let include_list: Vec<String> = vec![".txt"].into_iter().map(|s| s.to_string()).collect();
        

        let test_file = PathBuf::from("test.txt");
        create_file(&test_file);

        visit_drs(&get_dir(),
        &ignore_list,
        &include_list,
"// David Serrano",
&mut vec![]);
        // apply to this test file
        let path = get_dir().join(test_file);
        let contents = read_file(&path);
        // applies changes to file
        assert!(contents.len() > 0);  // it added // David Serrano
        assert_eq!(contents,"// David Serrano\nI want to stay the same!");

        // but dont apply to this file (since ignore_list contains /target path)
        let path = get_dir().join(PathBuf::from("target/test_2.txt"));

        assert!(read_file(&path).len() == 0);
        
        // contents after: and dont apply to random file
        let contents_after = read_file(&main_file_path);
        assert_eq!(contents_before, contents_after);
    }

    




}