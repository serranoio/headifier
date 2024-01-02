// Hello!
// Headifier 0.4.0
// David Serrano, Jan 2nd, 2024
// Made with love.
use std::env;
use std::fs::File;
use std::fs::{self};
use std::io::{Write, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

use wildmatch::WildMatch;

use crate::ui::app::{self, Mode};

// files are marked as single paths
// directories are marked as /
// * count everything
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

fn prepend_header(path: &PathBuf, header: &str, mode: &Mode) -> bool {
    let contents = read_file(path);

    let mut write_to_file = File::create(path).unwrap();

    let (contents, comment_count) = prepare_header(contents, header, mode);
    write!(write_to_file, "{}", contents).unwrap();

    let mut is_edited = false;

    if comment_count == 0 && *mode == Mode::Add {
        is_edited = true
    } else if *mode == Mode::Replace {
        is_edited = true
    }

    return is_edited 
}

// lets also add deleting all headers

// take every file,
// take your new header,
fn contains_header(contents: &String, mode: &Mode) -> (String, i32) {
// the only thing left to do is to remove the lines with the comments in the replace function

let mut comments = 0;
    let mut found_code = false;
    let new_contents = contents.split("\n")
    .map(|line| {
        if line.contains("//") {
            if !found_code {
                comments+=1;
                
                if *mode == Mode::Replace {
                    return "".to_owned();
                }
            }
        } else {
            found_code = true;
        }

        return format!("{}\n", line)
    }).collect();

    return (new_contents, comments)
}

pub fn prepare_header(contents: String, header: &str, mode: &Mode) -> (String, i32) {    
    // assume comments at beginning is header
    let (contents, comment_count) =  contains_header(&contents, mode);
    
    if *mode == Mode::Add && comment_count == 0 || *mode == Mode::Replace { // no comments
        // if file does not contain header-> add headers to all
        return (format!("{header}\n{contents}"), 0);
    } else if comment_count > 0 && *mode == Mode::Add {
        // if file contains header -> only bump version # and change date if git detects a change
        // or as of now, we dont add header
        return (format!("{contents}"), comment_count);
        // how would we find a date?
        // how would we get the version #
    }

    return (String::from("{contents}"), comment_count);
}

fn add_header_to_file(path: &PathBuf, ignore_list: &Vec<String>,
    include_list: &mut Vec<String>, header: &str,
    applied_list:  &mut Vec<String>,
    mode: &Mode) {
    let path_as_string = path.display().to_string().to_ascii_lowercase();
    
    let mut get_ignored = false;
    for ignore in ignore_list {
        let wildmatch = WildMatch::new(&ignore);
        if wildmatch.matches(&path_as_string) {
            get_ignored = true;
        }
    }
    
    if get_ignored {
        return; 
    }
    
    for include in include_list.clone() {    
        let wildmatch = WildMatch::new(&include);
        let path_as_string = path.display().to_string().to_ascii_lowercase();

        if wildmatch.matches(&path_as_string) && !applied_list.contains(&path_as_string) {
            let is_edited = prepend_header(path, header, mode);
            if is_edited {
                applied_list.push(path_as_string);
            }
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

pub fn app_interface(dir: &Path, 
ignore_list: &mut Vec<String>,
include_list: &mut Vec<String>,
header: &str,
 applied_list:  &mut Vec<String>,
mode: &Mode) {
     
     visit_drs(dir, ignore_list, include_list, header, applied_list, &mode)
    }
    
    fn visit_drs(dir: &Path,
        ignore_list: &mut Vec<String>,
        include_list: &mut Vec<String>, header: &str,
        applied_list:  &mut Vec<String>,
    mode: &Mode) {
        add_wildcards_to_path(include_list);
        add_wildcards_to_path(ignore_list);

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();

        let path = entry.path();

        if path.is_dir() {
            visit_drs(&path, ignore_list, include_list, header, applied_list, &mode);
        } else {
            add_header_to_file(&path, &ignore_list, include_list, header, applied_list, &mode);
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

pub fn add_wildcards_to_path(list: &mut Vec<String>) {
    for item in list {
        let mut last = String::from("");
        let mut first = String::from("");
        if !(item.chars().next().unwrap() == '*') {
            first = "*".to_owned()
        }

        if !(item.chars().last().unwrap() == '*') {
            last = "*".to_owned();
        }


    *item = format!("{}{}{}", first, item, last)
    }
}
#[cfg(test)]
mod tests {
    use std::env;
    use std::path::Path;
    use std::path::PathBuf;

    use wildmatch::WildMatch;

    use crate::ui::app::Mode;

    use super::add_wildcards_to_path;
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
    
    const CHANGE_CONTENTS: &str = "I want to change!";
    const IGNORE_ME: &str = "ignore me!";

fn create_file(test_file: &PathBuf, contents: &str) -> String {
        let mut handle = std::fs::File::create(test_file).unwrap();

        write!(handle, "{contents}").unwrap();

        contents.into()
}

    const ADD_TO_FILE: &str = "// David Serrano";

    #[test]
    pub fn test_visit_dirs() {
        // contents before
        let main_file_path = PathBuf::from("src/main.rs");

        let mut ignore_list: Vec<String> = vec![".git*", "/target", "cargo.toml", "cargo.lock", "README.md", "ignore.txt"].into_iter().map(|s| s.to_string()).collect();
        let mut include_list: Vec<String> = vec!["*.txt"].into_iter().map(|s| s.to_string()).collect();

        let test_file = PathBuf::from("test.txt");
        let ignore_test_file = get_dir().join(PathBuf::from("ignore.txt"));
        create_file(&test_file, CHANGE_CONTENTS);
        create_file(&ignore_test_file, IGNORE_ME);

        visit_drs(&get_dir(),
        &mut ignore_list,
        &mut include_list,
        ADD_TO_FILE,
        &mut vec![], &Mode::Add);
        // apply to this test file
        let path = get_dir().join(test_file);
        let contents = read_file(&path);
        // applies changes to file
        assert_eq!(contents, format!("{}\n{}",ADD_TO_FILE, CHANGE_CONTENTS));

        let path = get_dir().join(ignore_test_file);
        let contents = read_file(&path);
        assert_eq!(contents, format!("{}",IGNORE_ME));


        let mut include_list: Vec<String> = vec![".rs"].into_iter().map(|s| { s.to_string()}).collect();
        let mut ignore_list: Vec<String> = vec![".git*", "target"].into_iter().map(|s| { s.to_string()}).collect();


        visit_drs(&get_dir(), &mut ignore_list, &mut include_list, ADD_TO_FILE, &mut vec![], &Mode::Add);
        // but dont apply to this file
    }

    #[test]
    pub fn test_wild_cards() {
    let mut include_list: Vec<String> = vec!["*.txt", "target", "*json", "include.txt"].into_iter().map(|s| s.to_string()).collect();

    add_wildcards_to_path(&mut include_list);

    print!("{:?}", include_list);
    assert!(WildMatch::new(&include_list[0]).matches("random/folder/bacon.txt"));
    assert!(WildMatch::new(&include_list[0]).matches("random-folder/bacon.txt"));
    assert!(WildMatch::new(&include_list[0]).matches("hello/bacon.txt"));
    assert!(WildMatch::new(&include_list[0]).matches("bacon.txt"));
    assert!(WildMatch::new(&include_list[0]).matches("cool.txt"));
    assert!(WildMatch::new(&include_list[0]).matches("ok.txt"));

    assert!(WildMatch::new(&include_list[1]).matches("/target/ok.txt"));
    assert!(WildMatch::new(&include_list[1]).matches("/target/asdasd/ok.txt"));
    assert!(WildMatch::new(&include_list[1]).matches("/target/asdasd/oasdaisd.txt"));
    assert!(WildMatch::new(&include_list[1]).matches("/target/asdasd/oasdaisd.ts"));

    assert!(WildMatch::new(&include_list[2]).matches("launch.json"));
    assert!(WildMatch::new(&include_list[2]).matches("hello.json"));

    assert!(WildMatch::new(&include_list[3]).matches("include.txt"));
}




}