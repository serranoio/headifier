// David Serrano
// Headifier 0.1.0
// David Serrano, October 21st, 2023
// MIT License
// Made With Love ❤️
use std::{path::PathBuf};

use crate::core::{ list_git_ignore, get_dir, find_header};

pub const ONE: u8 = 49;
pub const TWO: u8 = 50;
pub const THREE: u8 = 51;
pub const FOUR: u8 = 51;

#[derive(Debug)]
pub enum WelcomeScreenOptions {
    HeaderScreen(HeaderScreenOptions),
    IgnoreScreen,
    IncludeScreen,
    Initial,
    Applied,
}

#[derive(Debug, Clone)]
pub enum HeaderScreenOptions {
    FromTextFile,
    New,
    Initial,
}

#[derive(Debug)]
pub struct ArrowPosition {
    pub welcome_arrow: usize,
    pub header_arrow: usize,
}

#[derive(Debug)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    // here is the string that will be applied to each screen
    pub string: String,
    pub intro_string: String,
    // cursor
    display_cursor_counter: u8,
    pub display_cursor: String,
    // cur screen
    pub screen: WelcomeScreenOptions,
    // global state
    pub header: String,
    pub ignore_list: Vec<String>,
    pub include_list: Vec<String>,

    pub applied_list: Vec<String>,
    // dir
    dir: PathBuf,

    // arrow
    pub arrow_positions: ArrowPosition,
    // size
    pub size: Size,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        // get ignore list
        let dir = get_dir();

        App {
            should_quit: false,
            string: "".into(),
            header: "".into(),
            intro_string: "".into(),
            display_cursor_counter: 0,
            display_cursor: String::from(""),
            screen: WelcomeScreenOptions::Initial,
            include_list: vec![],
            ignore_list: list_git_ignore(&dir),
            applied_list: vec![],
            dir,
            arrow_positions: ArrowPosition{welcome_arrow: 1, header_arrow: 1},
            size: Size { width: 0, height: 0 }
        }
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    
    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {

        if self.display_cursor_counter > 4 {
            self.display_cursor_counter = 0;
        } else if self.display_cursor_counter >= 2 {
            self.display_cursor = String::from("|");
        } else {
            self.display_cursor = String::from("");
        }
        self.display_cursor_counter += 1;
    }

    pub fn increment_arrow_position(&mut self) {
        if matches!(self.screen, WelcomeScreenOptions::Initial) {  // 3 options
            if  self.arrow_positions.welcome_arrow == 1 { return;}
            self.arrow_positions.welcome_arrow -= 1;       
        } else if matches!(self.screen, WelcomeScreenOptions::HeaderScreen(HeaderScreenOptions::Initial)) {
            if  self.arrow_positions.header_arrow == 1 { return;}
            
            self.arrow_positions.header_arrow -= 1;       
        }
    }
    
    pub fn decrement_arrow_position(&mut self) {
        if matches!(self.screen, WelcomeScreenOptions::Initial) {
            if  self.arrow_positions.welcome_arrow == 3 { return;}
            self.arrow_positions.welcome_arrow += 1;       
        } else if matches!(self.screen, WelcomeScreenOptions::HeaderScreen(HeaderScreenOptions::Initial)) {
            if  self.arrow_positions.header_arrow == 2 { return;}
            self.arrow_positions.header_arrow += 1;       
        }     
    }

    fn intro(&mut self) {
        if &self.string.len() > &0 {
            self.intro_string = "".into();   
        } else {
            match &self.screen {
                WelcomeScreenOptions::HeaderScreen(hs) => {
                    match hs {
                        HeaderScreenOptions::Initial => {
                            self.intro_string = "".into();
                        }, HeaderScreenOptions::FromTextFile => {
                            self.intro_string = "".into(); 
                        }, HeaderScreenOptions::New => {
                            self.intro_string = "Start writing to define your header!".into();
                        }
                    }
                }, WelcomeScreenOptions::IgnoreScreen => {
                    self.intro_string = "Could not find .gitignore / nothing present".into();
                }, WelcomeScreenOptions::IncludeScreen => {
                    self.intro_string = "Start writing to define the files to include!".into();
                }, WelcomeScreenOptions::Initial => {
                    self.intro_string = "".into();
                }, WelcomeScreenOptions::Applied => {
                    self.intro_string = self.applied_list.join("");
                }
                
            }   
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn add_char(&mut self, key: char) {
        self.string.push(key);

        self.display_cursor_counter = 2;
        self.intro()
    }

    pub fn remove_char(&mut self) {
        if self.string.len() == 0 {
            return;
        }

        self.string.pop();

        if self.string.len() == 0 {
            self.intro();
        }
    }

    pub fn change_step(&mut self, new_step: WelcomeScreenOptions) {  
        self.screen = new_step;
        // self.quit();
        match &self.screen {
            WelcomeScreenOptions::HeaderScreen(hs) => {
            match hs {
                HeaderScreenOptions::Initial => {
                    self.string = "".into();   
                }, HeaderScreenOptions::FromTextFile => {
                    match find_header(&self.dir) {
                        Some(file_contents) => {
                            self.string = file_contents;
                        },
                        None => {
                            self.intro_string = "Could not find header.txt.\nStart from empty file".into()
                        },
                    };
                }, HeaderScreenOptions::New => {},
            }
        },
        WelcomeScreenOptions::IgnoreScreen => {
            self.string = App::turn_vector_to_string(&self.ignore_list); 
        },
        WelcomeScreenOptions::IncludeScreen => {
            // on include screen, we need to  include include_list as string
            self.string = App::turn_vector_to_string(&self.include_list);
        },
        WelcomeScreenOptions::Initial => {},
        WelcomeScreenOptions::Applied => {},
    }


        if !matches!(&self.screen,
             WelcomeScreenOptions::HeaderScreen(HeaderScreenOptions::FromTextFile)) {
            self.intro()
        }
    }

    fn turn_vector_to_string(vec: &Vec<String>) -> String {
        vec.join("\n")
    }

    fn turn_string_to_vector(string: &String) -> Vec<String> {
        string.split_whitespace().map(|s| s.to_string()).collect()
    }

    pub fn apply_text(&mut self) {
        match &self.screen {
            WelcomeScreenOptions::HeaderScreen(hs) => {
                match hs {
                    HeaderScreenOptions::Initial => {

                    }, HeaderScreenOptions::FromTextFile => {
                        self.header = self.string.clone();
                        self.screen = WelcomeScreenOptions::Applied;
                        crate::core::app_interface(&self.dir, &mut self.ignore_list,
                            &mut self.include_list, &self.header, &mut self.applied_list);
                    }, HeaderScreenOptions::New => {
                        self.header = self.string.clone();
                        self.screen = WelcomeScreenOptions::Applied;
                        crate::core::app_interface(&self.dir, &mut self.ignore_list,
                            &mut self.include_list, &self.header, &mut self.applied_list);
                    },
                }
            },
            WelcomeScreenOptions::IgnoreScreen => {

                self.ignore_list = App::turn_string_to_vector(&self.string);
                self.screen = WelcomeScreenOptions::Initial;
             
            },
            WelcomeScreenOptions::IncludeScreen => {
                self.include_list = App::turn_string_to_vector(&self.string);
                self.screen = WelcomeScreenOptions::Initial;
            },
            WelcomeScreenOptions::Initial => {
            
            },
            WelcomeScreenOptions::Applied => {},
        }

        self.string = "".into();            
    }
}

mod tests {}
