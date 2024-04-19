use std::time::Duration;

use crate::app::{App, AppResult, Screen};
use crate::event::Event as AppEvent;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`]
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.screen {
        Screen::Start => match key_event.code {
            KeyCode::Esc => app.quit(),
            KeyCode::Char(ch) => {
                if (ch == 'c' || ch == 'C') && key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }

                app.enter_char(ch);
                app.screen = Screen::Typing;

                let _sender = app._sender.clone();
                tokio::spawn(async move {
                    let mut countdown = 0;
                    while countdown < 30 {
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        _sender.send(AppEvent::Tick).unwrap();
                        countdown += 1;
                    }
                });
            }
            _ => {}
        },
        Screen::Typing => match key_event.code {
            KeyCode::Esc => app.quit(),
            KeyCode::Backspace => app.remove_char(),
            KeyCode::Char(ch) => {
                if (ch == 'c' || ch == 'C') && key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }

                app.enter_char(ch);
            }
            _ => {}
        },
        Screen::Result => match key_event.code {
            KeyCode::Esc => app.quit(),
            KeyCode::Tab => app.reset(),
            _ => {}
        },
    }

    Ok(())
}
