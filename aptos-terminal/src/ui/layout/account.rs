use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, List, ListDirection, Paragraph, Widget,
    },
    Frame,
};

/// home page layout
pub fn render_account_layout(frame: &mut Frame) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(9), Constraint::Percentage(91)])
        .split(frame.size());
    /// working layout
    let bottom_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(main_layout[1]);
    let bottom_left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(bottom_layout[0]);
    /// search area
    frame.render_widget(
        Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" search by address / tx "),
        main_layout[0],
    );
    frame.render_widget(
        Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title("HOME1"),
        bottom_layout[0],
    );
    frame.render_widget(
        Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title("Block"),
        bottom_layout[1],
    );
    frame.render_widget(
        Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title("Block"),
        bottom_left_layout[0],
    );
    frame.render_widget(
        Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title("Block"),
        bottom_left_layout[1],
    );

    let items = ["Item 1", "Item 2", "Item 3"];
    let list: List = List::new(items)
        .block(Block::bordered().title("List"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::BottomToTop);
}
