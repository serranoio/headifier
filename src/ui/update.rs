use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::ui::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
  match key_event.code {
    KeyCode::Esc => app.quit(),
    KeyCode::Char(c) => {
      app.add_key_to_string(c)
    },
    KeyCode::Backspace => {
      app.remove_char()
    }
    KeyCode::Enter => {
      app.add_key_to_string('\n')
    },
    KeyCode::Right => {
      app.apply_header()
    }

    _ => {}
  };
}
