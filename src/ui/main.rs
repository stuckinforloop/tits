use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use super::body;
use super::top_bar;

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let parent_vertical_layout =
        Layout::vertical([Constraint::Length(1), Constraint::Percentage(75)]);
    let [top_bar_rect, body_rect] = parent_vertical_layout.areas(frame.size());

    top_bar::render(top_bar_rect, app, frame);
    body::render(body_rect, app, frame);
}
