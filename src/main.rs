use color_eyre::Result;
use ratatui;

mod game;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = game::Game::new(10,10).run(&mut terminal);
    ratatui::restore();
    Ok(result?)
}
