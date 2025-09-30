use rand::Rng;
use std::collections::VecDeque;

pub enum Direction { Up, Down, Left, Right }

pub struct Game {
    pub height: usize,
    pub width: usize,
    pub grid: Vec<Vec<usize>>,
    pub dir: Direction,
    pub score: i16,
    pub game_over: bool,
    snake: VecDeque<(usize,usize)>,
    food: (usize,usize),
}

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

    pub fn change_dir(&mut self, dir: Direction) {
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
}
