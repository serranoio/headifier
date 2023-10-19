use std::borrow::BorrowMut;

use crossterm::event::KeyCode;

use crate::core::write_all_headers;

#[derive(Debug)]
pub struct App {
  /// should the application exit?
  pub should_quit: bool,
  /// counter
  pub header: String,
  pub intro_string: String,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
      App{
        should_quit: false,
        header: "".into(),
        intro_string: "Start writing to define your header! Press Esc to quit ".into(),
      }
    }
    
    fn remove_header(&mut self) {
      self.intro_string = "".into();
    }
    
    fn reset_header(&mut self) {
      self.intro_string = "Start writing to define your header! Press Esc to quit ".into();
    }
  
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}
  
    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
      self.should_quit = true;
    }
  
    pub fn add_key_to_string(&mut self, key: char) {
      self.header.push(key);

      self.remove_header();
    }

    pub fn remove_char(&mut self) {
      if self.header.len() == 0 {
        return;
      }

      self.header.pop();

      if self.header.len() == 0 {
        self.reset_header();
      }
    }

    pub fn apply_header(&mut self) {
      write_all_headers(self.header.borrow_mut());

      self.quit();
    }


  }


  mod tests {
    
  }