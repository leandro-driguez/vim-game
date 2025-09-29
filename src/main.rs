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

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    
    let mut g = game::Game::new(10,10);  

    loop {
        clear();

        let frame = format!(
            "{}", ui::get_frame(&g)
        );

        print!("{frame}");
        io::stdout().flush()?; // force output now

        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                let new_dir = match key_event.code {
                    KeyCode::Char('h') => game::Direction::Left,
                    KeyCode::Char('j') => game::Direction::Down,
                    KeyCode::Char('k') => game::Direction::Up,
                    KeyCode::Char('l') => game::Direction::Right,
                    _ => g.dir,
                };
                g.dir = new_dir;
            }
        }
        thread::sleep(Duration::from_millis(500));
    }
    Ok(())
}

