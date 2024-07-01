use app::App;
use std::io;
mod app;
mod i18n;
mod router;
mod tui;
mod ui;
mod widget;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
