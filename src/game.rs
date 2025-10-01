use rand::Rng;
use ratatui::{
    DefaultTerminal, 
    Frame,
    style::{Stylize, Color},
    widgets::{
        canvas::{Canvas, Rectangle},
        Block,
        Widget,
    },
    text::Line,
    layout::Rect,
    buffer::Buffer,
    symbols::border,
};
use std::collections::VecDeque;
use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode}
};

pub enum Direction { Up, Down, Left, Right }

pub struct Game {
    pub height: usize,
    pub width: usize,
    pub grid: Vec<Vec<usize>>,
    pub dir: Direction,
    pub score: i16,
    pub game_over: bool,
    pub exit: bool,
    snake: VecDeque<(usize,usize)>,
    food: (usize,usize),
}

/// Methods that control the TUI
impl Game {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            self.update();
        }    
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        enable_raw_mode()?;

        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('h') => { self.change_dir(Direction::Left); },
                    KeyCode::Char('j') => { self.change_dir(Direction::Down); },
                    KeyCode::Char('k') => { self.change_dir(Direction::Up); },
                    KeyCode::Char('l') => { self.change_dir(Direction::Right); },
                    KeyCode::Char('q') => { self.exit = true; },
                    KeyCode::Char('r') => { self.restart(); },
                    _ => {},
                };
            }
        }

        disable_raw_mode()?; 

        Ok(())
    }
}

/// Implement Widget trait to render the game
impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let left = 0.0;
        let right = f64::from(area.width);
        let bottom = 0.0;
        let top = f64::from(area.height);

        let title = Line::from(" Snake Game ".bold());
        let instructions = Line::from(vec![
            " Left ".into(),
            "<h>".blue().bold(),
            " Right ".into(),
            "<l>".blue().bold(),
            " Up ".into(),
            "<k>".blue().bold(),   
            " Down ".into(),
            "<j>".blue().bold(),   
            " Restart ".into(),
            "<r>".blue().bold(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        Canvas::default()
            .block(block)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(|ctx| {
                for (i,j) in &self.snake {
                    ctx.draw(&Rectangle {
                        x: *i as f64,
                        y: *j as f64,
                        width: 1.0,
                        height: 1.0,
                        color: Color::Blue,
                    });
                }
                ctx.draw(&Rectangle{
                    x: self.food.0 as f64,
                    y: self.food.1 as f64,
                    width: 1.0,
                    height: 1.0,
                    color: Color::Red,
                });
            })
            .render(area, buf);
    }
}

/// Internal mechanics of the Snake Game
impl Game {
    pub fn new(h: usize, w: usize) -> Self {
        let mut rng = rand::rng();

        let x = rng.random_range(0..h);
        let y = rng.random_range(0..w);

        let mut i = rng.random_range(0..h);
        let mut j = rng.random_range(0..w);

        while x == i && y == j {
            i = rng.random_range(0..h);
            j = rng.random_range(0..w);
        }

        let mut snake: VecDeque<(usize,usize)> = VecDeque::new();
        snake.push_back((x,y));

        let mut grid: Vec<Vec<usize>> = vec![vec![0;w];h];
        grid[x][y] = 1;
        grid[i][j] = 2;
    
        return Self {
            height: h,
            width: w,
            grid: grid,
            snake: snake,
            food: (i,j),
            dir: Direction::Right,
            score: 0,
            game_over: false,
            exit: false,
        };
    }

    fn update(&mut self) {
        // get the next position of the snake
        let (i, j) = self.next_position();

        if i == self.food.0 && j == self.food.1 {
            // the next position is the food, eat and generate other
            // food position and increase score 
            let mut rng = rand::rng();

            let mut x = rng.random_range(0..self.height);
            let mut y = rng.random_range(0..self.width);

            // ensure the new food position is empty
            while self.grid[x][y] != 0 {
                x = rng.random_range(0..self.height);
                y = rng.random_range(0..self.width);
            }

            self.food = (x,y);
            self.grid[x][y] = 2;

            self.score += 1;
        }
        else { 
            // remove the tail of the snake
            let Some((x, y)) = self.snake.pop_front() else { todo!() };
            self.grid[x][y] = 0;
        }

        // check if the snake eat itself, in that case game over
        if self.grid[i][j] == 1 {
           self.game_over = true; 
        }
        
        // move forward to the next position
        self.snake.push_back((i,j));
        self.grid[i][j] = 1;
    }

    fn change_dir(&mut self, dir: Direction) {
       self.dir = dir; 
    }

    fn next_position(&self) -> (usize,usize) {
        let Some((i,j)) = self.snake.back() else { todo!() };
        match self.dir {
            Direction::Up    => ((*i + self.height - 1) % self.height, *j),
            Direction::Down  => ((*i + 1) % self.height, *j),
            Direction::Left  => (*i, (*j + self.width - 1) % self.width),
            Direction::Right => (*i, (*j + 1) % self.width),
        }
    }

    fn restart(& mut self) {
        *self = Game::new(self.height, self.width);
    }
}
