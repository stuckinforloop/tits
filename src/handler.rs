use std::time::Duration;

use crate::app::{App, AppResult, Screen};
use crate::event::Event as AppEvent;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`]
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.screen {
        Screen::Start => match key_event.code {
            KeyCode::Esc => app.quit(),
            KeyCode::Enter => {
                app.screen = Screen::Typing;
            }
            KeyCode::Char(ch) => {
                if (ch == 'c' || ch == 'C') && key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            _ => {}
        },
        Screen::Typing => match key_event.code {
            KeyCode::Esc => {
                app.quit();
            }
            KeyCode::Backspace => {
                if app.is_typing {
                    app.remove_char();
                }
            }
            KeyCode::Char(ch) => {
                if (ch == 'c' || ch == 'C') && key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }

                app.enter_char(ch);
                if !app.is_typing {
                    let _sender = app._sender.clone();
                    tokio::spawn(async move {
                        let mut countdown = 0;
                        while countdown < 30 {
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            _sender.send(AppEvent::Tick).unwrap();
                            countdown += 1;
                        }
                    });
                    app.is_typing = true;
                }
            }
            _ => {}
        },
        Screen::Result => match key_event.code {
            KeyCode::Esc => app.quit(),
            KeyCode::Tab => {
                app.reset();
                app.screen = Screen::Typing;
            }
            KeyCode::Char(ch) => {
                if (ch == 'c' || ch == 'C') && key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }

                if ch == 'h' || ch == 'H' {
                    app.reset();
                }
            }
            _ => {}
        },
    }

    Ok(())
}
