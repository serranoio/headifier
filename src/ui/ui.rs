use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
  };
  
  use crate::{ui::app::App, ui::tui::Frame};
  
  pub fn render(app: &mut App, f: &mut Frame) {

    f.render_widget(
      Paragraph::new(format!(
        "{}{}",
        app.intro_string,
        app.header
      ))
      .block(
        Block::default()
          .title("header.txt")
          .title_alignment(Alignment::Center)
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Yellow))
      .alignment(Alignment::Left),
      f.size(),
    )
  }