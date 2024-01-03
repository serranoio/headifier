// Headifier 0.2.0
// David Serrano
// January 3rd, 2024
// Made with love <3


use std::ops::Index;
use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect, Layout, Constraint, Direction},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
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

fn text_rect(r: Rect,  decide: usize, orientation: Direction) -> Rect {
    let popup_layout = Layout::default()
      .direction(orientation)
      .constraints([
        Constraint::Percentage(75),
        Constraint::Percentage(25),
      ])
      .split(r);


      popup_layout[decide]
}
      
fn write_screen(app: &mut App, f: &mut Frame, title: &str, instructions: &str) {
  let orientation = change_orientation(app, true);
  
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
    text_rect(f.size(), 0, orientation)
  ); 
 
  f.render_widget(
    Paragraph::new(format!(
        "Press Esc to go back\n{}", instructions))
    .block(
        Block::default()
            .title("instructions")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
          )
          .style(Style::default().fg(Color::Indexed(48)))
          .alignment(Alignment::Left),
          text_rect(f.size(), 1, orientation)
  ) 
}

fn change_orientation(app: &mut App, _is_options_screen: bool) -> Direction {

  if app.size.width  > app.size.height * 2 {
    return Direction::Horizontal
  };
  Direction::Vertical
}

fn applied_screen(app: &mut App, f: &mut Frame) {

  let all = app.applied_list
  .clone()
  .into_iter()
  .map(|mut s|{ s.push('\n'); s})
.collect::<Vec<String>>()
.join("");

  f.render_widget(
    Paragraph::new(all)
    .block(
        Block::default()
            .title("Applied!")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Indexed(208)))
    .alignment(Alignment::Left),
    f.size()
  );

}

fn options_screen(app: &mut App, f: &mut Frame, title: &str, options: String, instructions: String) {
  let orientation = change_orientation(app, true);
  
  f.render_widget(
    Paragraph::new(options)
    .block(
        Block::default()
            .title(title.to_string())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Indexed(208)))
    .alignment(Alignment::Left),
    text_rect( f.size(), 0, orientation)
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
          text_rect( f.size(), 1, orientation)
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

  let instructions = format!("1 to move to create header screen
2 to set list of files to ignore
3 to set list of files to include.
Or, use up and down arrow keys to navigate,
and press Enter to advance");

  options_screen(app, f, "Headifier", options, instructions)
}

pub fn header_screen(app: &mut App, f: &mut Frame) {
  let options = vec!("From text file", "New");
  let options = make_list(options, app.arrow_positions.header_arrow);

  let instructions = format!("1 to use contents of header.txt base of as header
2 to write new header in terminal.
Or, use Up and Down arrow keys to navigate,
and press Enter to advance");

options_screen(app, f, "Header Options", options, instructions);

}

const INSTRUCTIONS: &str = "Right arrow to add headers to files without headers\nLeft arrow to add headers to ALL headers (replace)";
pub fn new_header_screen(app: &mut App, f: &mut Frame) {
  write_screen(app, f, "New Header", INSTRUCTIONS); 
}

pub fn from_text_header_screen(app: &mut App, f: &mut Frame) {
  write_screen(app, f, "header.txt", INSTRUCTIONS);
}

pub fn ignore_screen(app: &mut App, f: &mut Frame) {
  let instructions = "Right arrow to add headers to files without headers";
  write_screen(app, f, "List Of Files To Ignore", instructions);
}

pub fn include_screen(app: &mut App, f: &mut Frame) {
  let instructions = "Right arrow to add headers to files without headers";
  write_screen(app, f, "List Of Files To Include", instructions);
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
      }, WelcomeScreenOptions::Applied => {
          applied_screen(app, f);
      }
    
    }
}