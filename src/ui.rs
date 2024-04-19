use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Screen};

/// Renders the user interface widgets.
pub fn render(app: &mut App, f: &mut Frame) {
    let vertical = Layout::vertical([
        Constraint::Length(2),
        Constraint::Percentage(75),
        Constraint::Min(2),
    ]);
    let helper_vertical = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]);
    let horizontal = Layout::horizontal([Constraint::Min(2), Constraint::Min(2)]);

    let [helper, input, results] = vertical.areas(f.size());
    let [quit, reset] = helper_vertical.areas(helper);
    let [wpm, timer] = horizontal.areas(results);

    let quit_msg = Paragraph::new(Text::from(Line::from(vec![
        "Press ".into(),
        "ESC ".bold(),
        "to quit".into(),
    ])))
    .style(Style::default().fg(Color::DarkGray))
    .alignment(Alignment::Right);
    f.render_widget(quit_msg, quit);

    if app.screen == Screen::Result {
        let reset_msg = Paragraph::new(Text::from(Line::from(vec![
            "Press ".into(),
            "TAB ".bold(),
            "to reset".into(),
        ])))
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Right);
        f.render_widget(reset_msg, reset);
    }

    let input_msg = {
        let title: &str;

        match app.screen {
            Screen::Start => title = " Start Typing...! ",
            Screen::Typing => title = " Listening... ",
            Screen::Result => title = " Your Score... ",
        }

        Paragraph::new(Text::from(
            Line::from(app.span_vec.clone()).style(Style::default()),
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::symmetric(3, 2))
                .title(title)
                .title_alignment(Alignment::Center),
        )
        .wrap(Wrap {
            ..Default::default()
        })
    };
    f.render_widget(input_msg, input);

    let wpm_msg = {
        let wpm = if app.screen == Screen::Result {
            (app.keys_pressed / 5.0) / 0.5
        } else {
            0.0
        };
        Paragraph::new(wpm.to_string())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" WPM ")
                    .title_alignment(Alignment::Center),
            )
            .style(Style::default())
            .centered()
            .bold()
    };
    f.render_widget(wpm_msg, wpm);

    let timer_msg = {
        let time_left = 30 - app.countdown;
        Paragraph::new(time_left.to_string())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Time Left ")
                    .title_alignment(Alignment::Center),
            )
            .style(Style::default())
            .centered()
            .bold()
    };
    f.render_widget(timer_msg, timer);
}
