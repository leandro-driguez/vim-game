use rand::Rng;
use std::collections::VecDeque;

pub enum Direction { Up, Down, Left, Right }

pub struct Game {
    height: u8,
    width: u8,
    grid: Vec<Vec<u8>>,
    snake: VecDeque<(u8,u8)>,
    food: (u8,u8),
    dir: Direction,
    score: i16,
}

impl Game {
    pub fn new(h: u8, w: u8) -> Self {
        let mut rng = rand::rng();

        let x = rng.random_range(0..h);
        let y = rng.random_range(0..w);

        let mut i = rng.random_range(0..h);
        let mut j = rng.random_range(0..w);

        while x == i && y == j {
            i = rng.random_range(0..h);
            j = rng.random_range(0..w);
        }

        let snake: VecDeque<(u8,u8)> = VecDeque::new();
        snake.push_back((x,y));

        let mut grid: Vec<Vec<u8>> = vec![vec![0;w.into()];h.into()];
        grid[x][y] = 1;
        grid[i][j] = -1;
    
        return Self {
            height: h,
            width: w,
            grid: grid,
            snake: snake,
            food: (i,j),
            dir: Direction::Right,
            score: 0,
        };
    }

    pub fn update(&mut self) {
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
            self.grid[x][y] = -1;

            self.score += 1;
        }
        else { 
            // remove the tail of the snake
            let Some((x, y)) = self.snake.pop_front();
            self.grid[x][y] = 0;
        }
        
        // move forward to the next position
        self.snake.push_back((i,j));
        self.grid[i][j] = 1;
    }

    pub fn change_dir(&mut self, dir: Direction) {
       self.dir = dir; 
    }

    fn next_position(&self) -> (u8,u8) {
        let Some((i,j)) = self.snake.back();
        match self.dir {
            Direction::Up    => (0.max(*i - 1), *j),
            Direction::Down  => ((*i + 1) % self.height, *j),
            Direction::Left  => (*i, 0.max(*j - 1)),
            Direction::Right => (*i, (*j + 1) % self.width),
        }
    }
}
