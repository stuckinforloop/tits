use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};

use crate::app::{App, Screen};

fn get_right_top_bar_spans<'a>(app: &mut App) -> Vec<Span<'a>> {
    match app.screen {
        _ => vec!["Press ".into(), "ESC ".bold(), "to exit ".into()],
    }
}

fn get_left_top_bar_spans<'a>(app: &mut App) -> Vec<Span<'a>> {
    let time_left = (30 - app.countdown).to_string();
    match app.screen {
        Screen::Start => vec![],
        Screen::Typing => vec![
            "Time Left: ".into(),
            time_left.bold(),
            " Press ".into(),
            "TAB ".bold(),
            "to restart session".into(),
        ],
        Screen::Result => vec![
            "Press ".into(),
            "TAB ".bold(),
            "to start new session ".into(),
        ],
    }
}

fn get_top_bar_paragraph<'a>(spans: Vec<Span<'a>>) -> Paragraph<'a> {
    Paragraph::new(Text::from(Line::from(spans))).style(Style::default().fg(Color::DarkGray))
}

pub fn render(top_bar_rect: Rect, app: &mut App, frame: &mut Frame) {
    let constraints = [Constraint::Percentage(50), Constraint::Percentage(50)];
    let top_bar_layout = Layout::horizontal(constraints);
    let [top_bar_left_rect, top_bar_right_rect] = top_bar_layout.areas(top_bar_rect);

    let right_vector = get_right_top_bar_spans(app);
    let right_paragraph = get_top_bar_paragraph(right_vector).alignment(Alignment::Right);
    frame.render_widget(right_paragraph, top_bar_right_rect);

    let left_vector = get_left_top_bar_spans(app);
    let left_paragraph = get_top_bar_paragraph(left_vector).alignment(Alignment::Left);
    frame.render_widget(left_paragraph, top_bar_left_rect);
}
