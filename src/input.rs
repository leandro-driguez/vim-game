use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};

use crate::game;

pub enum CustomEvent {
    Direction(crate::game::Direction),
    Nothing,
    Exit,
}

pub fn handle_key_events() -> Result<CustomEvent, io::Error> {
    let mut custom_event = CustomEvent::Nothing;

    enable_raw_mode()?;
    if event::poll(std::time::Duration::from_millis(500))? {
        if let Event::Key(key_event) = event::read()? {
            custom_event = match key_event.code {
                KeyCode::Char('h') => CustomEvent::Direction(crate::game::Direction::Left),
                KeyCode::Char('j') => CustomEvent::Direction(crate::game::Direction::Down),
                KeyCode::Char('k') => CustomEvent::Direction(crate::game::Direction::Up),
                KeyCode::Char('l') => CustomEvent::Direction(crate::game::Direction::Right),
                KeyCode::Char('q') => CustomEvent::Exit,
                _ => CustomEvent::Nothing,
            };
        }
    }
    disable_raw_mode();   
    
    return Ok(custom_event);
}
