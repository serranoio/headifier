// Headifier 0.1.0
// David Serrano, October 21st, 2023
// MIT License
// Made With Love ❤️
use crossterm::event::{KeyCode, KeyEvent};

use crate::ui::app::App;

use super::app::{WelcomeScreenOptions, HeaderScreenOptions, ONE, TWO, THREE};

// ONLY LET QUIT AT BEGINNING PAGE, OTHERWISE, ESCAPE GOES BACK
fn editable_file(app: &mut App, key_event: KeyEvent,
    back_screen: WelcomeScreenOptions) {
    match key_event.code {
        KeyCode::Esc => app.change_step(back_screen),  // GO BACK
        KeyCode::Char(c) => {
          app.add_char(c)
        },
        KeyCode::Backspace => {
          app.remove_char()
        }
        KeyCode::Enter => {
          app.add_char('\n')
        },
        KeyCode::Right => {
          app.apply_text()  
        }
    
        _ => {}
      };
}

fn ignore_screen(app: &mut App, key_event: KeyEvent) {
    editable_file(app, key_event, WelcomeScreenOptions::Initial);
    // were here at an editable file of ignore
}

fn include_screen(app: &mut App, key_event: KeyEvent) {
    editable_file(app, key_event, WelcomeScreenOptions::Initial);
    // were here at an editable file of include
}

fn applied_screen(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => app.change_step(WelcomeScreenOptions::Initial),
        _ => {}
    };
}

fn header_screen(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => app.change_step(WelcomeScreenOptions::Initial),
        KeyCode::Char(c) => {
            if c == ONE as char {  // choose option 1
                app.change_step(WelcomeScreenOptions::HeaderScreen(HeaderScreenOptions::FromTextFile))
            } else if c == TWO as char {  // choose option 2
                app.change_step(WelcomeScreenOptions::HeaderScreen(HeaderScreenOptions::New))
            }
        }, KeyCode::Up => {
            app.increment_arrow_position();
        },
        KeyCode::Down => {
            app.decrement_arrow_position();
        },
        KeyCode::Enter => {
            if app.arrow_positions.header_arrow == 1 {
                app.change_step(WelcomeScreenOptions::HeaderScreen(HeaderScreenOptions::FromTextFile))
            } else if app.arrow_positions.header_arrow == 2 {
                app.change_step(WelcomeScreenOptions::HeaderScreen(HeaderScreenOptions::New))
            }
        }, 
        _ => {}
    };
}

fn initial_screen(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => app.quit(),
        KeyCode::Char(c) => {
            if c == ONE as char {  // choose option 1
                app.change_step(WelcomeScreenOptions::HeaderScreen(HeaderScreenOptions::Initial))
            } else if c == TWO as char {  // choose option 2
                app.change_step(WelcomeScreenOptions::IgnoreScreen) 
            } else if c == THREE as char { // choose option 3
                app.change_step(WelcomeScreenOptions::IncludeScreen) 
            }
        }, KeyCode::Up => {
            app.increment_arrow_position();
        },
        KeyCode::Down => {
            app.decrement_arrow_position();
            
        },
        KeyCode::Enter => {
            if app.arrow_positions.welcome_arrow == 1 {
                app.change_step(WelcomeScreenOptions::HeaderScreen(HeaderScreenOptions::Initial))
            } else if app.arrow_positions.welcome_arrow == 2 {
                app.change_step(WelcomeScreenOptions::IgnoreScreen) 
            } else if app.arrow_positions.welcome_arrow == 3 {
                app.change_step(WelcomeScreenOptions::IncludeScreen) 
            }
        }, 
        _ => {}
    };
}

fn new_header_screen(app: &mut App, key_event: KeyEvent) {
editable_file(app, key_event, WelcomeScreenOptions::Initial)
}

fn from_file_header_screen(app: &mut App, key_event: KeyEvent) {
editable_file(app, key_event, WelcomeScreenOptions::Initial)
}

fn header_screen_options(app: &mut App, key_event: KeyEvent, hs: HeaderScreenOptions) {
    match hs {
        HeaderScreenOptions::Initial => header_screen(app, key_event),  
        HeaderScreenOptions::FromTextFile => from_file_header_screen(app, key_event),
        HeaderScreenOptions::New => new_header_screen(app, key_event),
    }
}

pub fn update(app: &mut App, key_event: KeyEvent) {
    match &app.screen {
        WelcomeScreenOptions::HeaderScreen(ref hs) => {
            header_screen_options(app, key_event, hs.clone())
        }, WelcomeScreenOptions::IgnoreScreen => {
            ignore_screen(app, key_event)
        }, WelcomeScreenOptions::IncludeScreen => {
            include_screen(app, key_event)
        }, WelcomeScreenOptions::Initial => {
            initial_screen(app, key_event)

        }, WelcomeScreenOptions::Applied => {
            applied_screen(app, key_event)
        }
    }
}


fn options_resizing(app: &mut App, width: u16, height: u16) {
    
app.set_size(super::app::Size { width , height })    

}


// SCREEN RESIZING
pub fn update_screen(app: &mut App, width: u16, height: u16) {
    match &app.screen {
        WelcomeScreenOptions::HeaderScreen(ref hs) => {
            match &hs {
                HeaderScreenOptions::FromTextFile => {

                },
                HeaderScreenOptions::Initial => {

                    options_resizing(app, width, height);
                },
                HeaderScreenOptions::New => {

                }
            }
        },
        WelcomeScreenOptions::IgnoreScreen => {
            
        },
        WelcomeScreenOptions::IncludeScreen => {
            
        },
        WelcomeScreenOptions::Initial => {
            options_resizing(app, width, height);

        } WelcomeScreenOptions::Applied => {

        }
    }
}






