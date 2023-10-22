// Headifier 0.1.0
// David Serrano, October 21st, 2023
// MIT License
// Made With Love ❤️
use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect, Layout, Constraint, Direction},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget, block::Title},
};

use crate::{ui::app::App, ui::tui::Frame};
use super::app::{WelcomeScreenOptions, HeaderScreenOptions};
pub struct Instructions {
  // Custom widget properties
  content: String,
}

impl Widget for Instructions {
  fn render(self, area: Rect, buf: &mut Buffer) {
      // Rendering logic goes here
    buf.set_string(area.right()/2, area.top(),&self.content, Style::default().fg(Color::Green))
  }
}

fn text_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
        Constraint::Percentage(0),
        Constraint::Percentage(80),
        Constraint::Percentage(100), 
      ])
      .split(r);
  
    Layout::default()
      .direction(Direction::Horizontal)
      .constraints([
        Constraint::Percentage(0),
        Constraint::Percentage(100),
        Constraint::Percentage((100 - percent_x) / 2),
      ])
      .split(popup_layout[1])[1]
}

fn bottom_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
        Constraint::Percentage(80),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
      ])
      .split(r);
  
    Layout::default()
      .direction(Direction::Horizontal)
      .constraints([
        Constraint::Percentage(0),
        Constraint::Percentage(100),
        Constraint::Percentage((100 - percent_x) / 2),
      ])
      .split(popup_layout[1])[1]
}

fn write_screen(app: &mut App, f: &mut Frame, title: &str) {
  f.render_widget(
    Paragraph::new(format!(
        "{}{}{}",
        app.intro_string, app.string, app.display_cursor,
    ))
    .block(
        Block::default()
            .title(title)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Indexed(208)))
    .alignment(Alignment::Left),
    text_rect(f.size(), 30, 100)
  ); 
 
  f.render_widget(
    Paragraph::new(format!(
        "Press Esc to go back\nPress right arrow to apply changes"
    ))
    .block(
        Block::default()
            .title("instructions")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
          )
          .style(Style::default().fg(Color::Indexed(48)))
          .alignment(Alignment::Left),
          bottom_rect(f.size(), 30, 30)
  ) 
}

fn options_screen(app: &mut App, f: &mut Frame, title: &str, options: String, instructions: String) {
  f.render_widget(
    Paragraph::new(options)
    .block(
        Block::default()
            .title("header.txt")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Indexed(208)))
    .alignment(Alignment::Left),
    text_rect(f.size(), 30, 100)
  );
 
  f.render_widget(
    Paragraph::new(instructions)
    .block(
        Block::default()
            .title("instructions")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
          )
          .style(Style::default().fg(Color::Indexed(48)))
          .alignment(Alignment::Left),
          bottom_rect(f.size(), 30, 30)
  );
}

fn make_list(options: Vec<&str>, arrow_position: usize) -> String {

  options
  .into_iter()
  .enumerate()
  .map(|(i, option)| {
    let mut arrow = "  ";
    if arrow_position == i + 1 {
      arrow = "->";
    }
  
    format!("{} {}. {}\n", arrow, i+1, option)
  }).collect::<Vec<String>>().join("")

}  

pub fn initial_screen(app: &mut App, f: &mut Frame) {
  let options = vec!["Create header", "Set Ignore List", "Set Include List"];
  let options = make_list(options, app.arrow_positions.welcome_arrow);

  let instructions = format!("Press 1 to move to create header screen
Press 2 to set list of files to ignore
Press 3 to set list of files to include.
Or, use up and down arrow keys to navigate, and press Enter to advance");

  options_screen(app, f, "Headifier", options, instructions)
}

pub fn header_screen(app: &mut App, f: &mut Frame) {
  let options = vec!("From text file", "New");
  let options = make_list(options, app.arrow_positions.header_arrow);

  let instructions = format!("Press 1 to use contents of header.txt base of as header
Press 2 to write new header in terminal.
Or, use Up and Down arrow keys to navigate, and press Enter to advance");

  options_screen(app, f, "Header Options", options, instructions);
  

}

pub fn new_header_screen(app: &mut App, f: &mut Frame) {
  write_screen(app, f, "New Header"); 
}

pub fn from_text_header_screen(app: &mut App, f: &mut Frame) {
  write_screen(app, f, "header.txt");
}

pub fn ignore_screen(app: &mut App, f: &mut Frame) {
  write_screen(app, f, "List Of Files To Ignore");
}

pub fn include_screen(app: &mut App, f: &mut Frame) {
  write_screen(app, f, "List Of Files To Include");
}

pub fn render(app: &mut App, f: &mut Frame) {
    app.tick();

    match &app.screen {
      WelcomeScreenOptions::HeaderScreen(hs) => {
        match &hs {
          HeaderScreenOptions::Initial => header_screen(app, f),
          HeaderScreenOptions::New => new_header_screen(app, f),
          HeaderScreenOptions::FromTextFile => from_text_header_screen(app, f),
        }
      },
      WelcomeScreenOptions::IgnoreScreen => {
        ignore_screen(app,f);
      },
      WelcomeScreenOptions::IncludeScreen => {
        include_screen(app,f);
      },
      WelcomeScreenOptions::Initial => {
          initial_screen(app,f);
      },
    }
 
}
