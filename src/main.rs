use std::{io::{self, Write}, thread, time::Duration};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};

mod game;
mod ui;


fn clear() {
    // ANSI: clear whole screen + move cursor to top left
    print!("\x1b[2J\x1b[H");
}

fn main() -> crossterm::Result<()> {
    enable_raw_mode()?;
    
    let mut g = game::Game::new(100,100);  

    loop {
        clear();

        let frame = format!(
            ui::get_frame(g)
        );

        print!("{frame}");
        io::stdout().flush()?; // force output now

        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('h') => g.change_dir(game::Direction::Left),
                    KeyCode::Char('j') => g.change_dir(game::Direction::Down),
                    KeyCode::Char('k') => g.change_dir(game::Direction::Up),
                    KeyCode::Char('l') => g.change_dir(game::Direction::Right),
                }
            }
        }
        thread::sleep(Duration::from_millis(500));
    }
    Ok(())
}

