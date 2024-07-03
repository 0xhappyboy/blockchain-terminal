use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, router::Router};

pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Left => app.decrement_counter(),
        KeyCode::Right => app.increment_counter(),
        KeyCode::Char('1') => app.router = Router::HOME,
        KeyCode::Char('2') => app.router = Router::ACCOUNT,
        KeyCode::Char('3') => app.router = Router::TRANSACTION,
        KeyCode::Char('4') => app.a = 123,
        _ => {}
    }
}
