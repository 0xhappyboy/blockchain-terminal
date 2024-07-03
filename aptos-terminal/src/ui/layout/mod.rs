use account::render_account_layout;
use home::render_home_layout;
use ratatui::Frame;
use transaction::render_transaction_layout;

use crate::{app::App, router::Router};

mod account;
pub mod home;
mod transaction;

pub fn render_layout(frame: &mut Frame, app: &App) {
    match app.router {
        Router::HOME => render_home_layout(frame,app),
        Router::ACCOUNT => render_account_layout(frame),
        Router::TRANSACTION => render_transaction_layout(frame),
    }
}
