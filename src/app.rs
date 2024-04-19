use std::error;

use ratatui::{
    style::{self, Color, Style, Stylize},
    text::Span,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{event::Event, utils::gen_text};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]
pub enum Screen {
    Start,
    Typing,
    Result,
}

enum CursorMovement {
    Forwards,
    Backwards,
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    pub text: String,
    pub span_vec: Vec<Span<'static>>,
    pub cursor_idx: usize,
    pub countdown: usize,
    pub keys_pressed: f64,
    pub exit: bool,
    pub _sender: UnboundedSender<Event>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(_sender: UnboundedSender<Event>) -> Self {
        let text = gen_text();

        let span_vec = string_to_spans(&text);

        let mut app = Self {
            screen: Screen::Start,
            text,
            span_vec,
            cursor_idx: 0,
            keys_pressed: 0.0,
            countdown: 0,
            exit: false,
            _sender,
        };

        app.paint_cursor(0, true);

        return app;
    }

    /// Handles timer tick event
    pub fn tick(&mut self) {
        self.countdown += 1;
        if self.countdown == 30 {
            self.screen = Screen::Result
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.exit = true;
    }

    pub fn enter_char(&mut self, ch: char) {
        if let Some(equal) = self.char_match(ch) {
            if equal {
                self.keys_pressed += 1.0;
            }
            self.update_span(self.cursor_idx, equal);
            self.move_cursor(CursorMovement::Forwards);
            self.paint_cursor(self.cursor_idx, true);
        };
    }

    /// Move cursor position
    fn move_cursor(&mut self, movement: CursorMovement) {
        match movement {
            CursorMovement::Forwards => {
                self.cursor_idx = self.cursor_idx.saturating_add(1);
            }
            CursorMovement::Backwards => {
                if self.cursor_idx == 0 {
                    return;
                }

                self.cursor_idx = self.cursor_idx.saturating_sub(1);
            }
        }
    }

    /// basically handle backspace
    pub fn remove_char(&mut self) {
        self.paint_cursor(self.cursor_idx, false);
        self.move_cursor(CursorMovement::Backwards);
        self.paint_cursor(self.cursor_idx, true);
    }

    /// check if the input matches the character at current index
    fn char_match(&self, want: char) -> Option<bool> {
        self.span_vec.get(self.cursor_idx).map(|span| {
            let got = span.to_string().chars().next().unwrap_or('\n');
            if got == want {
                return true;
            } else {
                return false;
            }
        })
    }

    fn update_span(&mut self, idx: usize, matched: bool) {
        let color = if matched { Color::Green } else { Color::Red };
        let span = self.span_vec.get(idx);
        if let Some(span) = span {
            let span = span.clone();
            self.span_vec[idx] = span.style(Style::new().fg(color)).not_underlined();
        }
    }

    fn paint_cursor(&mut self, idx: usize, paint: bool) {
        if idx > self.span_vec.len() - 1 {
            return;
        }

        let span = self.span_vec.get(idx);
        if let Some(span) = span {
            let span = span.clone();
            match paint {
                true => self.span_vec[idx] = span.style(Style::new()).underlined(),
                false => self.span_vec[idx] = span.style(Style::new()).not_underlined(),
            }
        }
    }

    pub fn reset(&mut self) {
        self.screen = Screen::Start;
        self.text = gen_text();
        self.span_vec = string_to_spans(&self.text);
        self.cursor_idx = 0;
        self.keys_pressed = 0.0;
        self.countdown = 0;
    }
}

fn string_to_spans(text: &String) -> Vec<Span<'static>> {
    let span_vec: Vec<Span<'static>> = text
        .chars()
        .map(|f| Span::styled(f.to_string(), Style::new().fg(style::Color::DarkGray)))
        .collect();

    span_vec
}
