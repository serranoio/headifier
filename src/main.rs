use std::error::Error;
use std::os;
use std::env;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{ Write};

use std::io::BufReader;
use std::io::Read;



fn header() -> String {

    "// David Serrano, 2023
// Accential
// Made with Love
// Enjoy.\n".into()
}

fn add_header_to_file(path: &PathBuf) {
    if path.display().to_string().contains("header.txt") {
        let file = File::open(path).unwrap();
        
        let mut buf_reader = BufReader::new(file);  
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
         
        
        let mut write_to_file = File::create(path).unwrap();
        
    
        write!(write_to_file, "{}{contents}", header());
    }
}

fn visit_drs(dir: &Path, ignore_list: &Vec<String>) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();

        let path = entry.path();

        if path.is_dir() {
            visit_drs(&path, &ignore_list);
        } else {
            add_header_to_file(&path);
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
fn list_git_ignore(dir: &Path) -> Vec<String> {
    find_get_ignore(dir);

    vec![]    
}

fn main() {
    let path = env::current_dir();

    let path_buf = match path {
        Ok(p) => {
            println!("{}", p.display());

            p
        },
        Err(e) => {
            panic!("No current directory{e}");
        },
    };

    let ignore_list = list_git_ignore(&path_buf);

    visit_drs(&path_buf, &ignore_list);

}
