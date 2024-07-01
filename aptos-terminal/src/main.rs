use app::App;
use std::io;
mod app;
mod tui;
mod widget;
mod layout;
mod i18n;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}

