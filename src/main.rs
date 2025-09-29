use std::{io::{self, Write}, thread, time::Duration};


mod game;
mod ui;
mod input;


fn clear() {
    // ANSI: clear whole screen + move cursor to top left
    print!("\x1b[2J\x1b[H");
}

fn main() -> io::Result<()> {
    
    let mut g = game::Game::new(10,10);  

    loop {
        clear();

        g.update();

        let frame = format!(
            "{}", ui::get_frame(&g)
        );

        print!("{frame}");
        io::stdout().flush()?; // force output now

        if let Ok(event) = input::handle_key_events() {
            match event {
                input::CustomEvent::Direction(dir) => { g.dir = dir; }
                input::CustomEvent::Exit => { break; }
                _ => { continue; }
            };
        }
    }
    Ok(())
}

