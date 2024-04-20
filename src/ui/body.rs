use ratatui::{
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Screen};

fn get_paragraph<'a>(title: &'a str, text: Vec<Span<'a>>) -> Paragraph<'a> {
    Paragraph::new(Text::from(Line::from(text).style(Style::default())))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::symmetric(3, 2))
                .title(title)
                .title_alignment(Alignment::Center),
        )
        .wrap(Wrap { trim: true })
}

pub fn render(body_rect: Rect, app: &mut App, frame: &mut Frame) {
    let paragraph = match app.screen {
        Screen::Start => get_paragraph(
            " (Touch) Typing In Terminal Space ",
            vec!["Press ".into(), "Enter".bold(), " to start".into()],
        )
        .alignment(Alignment::Center),

        Screen::Typing => {
            let title = if app.is_typing { "" } else { " Start Typing! " };
            get_paragraph(title, app.span_vec.clone())
        }

        Screen::Result => {
            let words_per_minute = ((app.key_count / 5.0) / 0.5).to_string();
            get_paragraph(
                " Your Score ",
                vec!["Words Per Minute ".into(), words_per_minute.into()],
            )
            .alignment(Alignment::Center)
        }
    };
    frame.render_widget(paragraph, body_rect);
}
