use crate::game::Game;

pub fn get_frame(g: &Game) -> String {
    let mut frame = String::new();

    for i in 0..g.height {
        for j in 0..g.width {
            let cell = match g.grid[i][j] {
                0 => ' ',
                1 => 'o',
                2 => 'x',
                _ => '?',
            };
            frame.push(cell);
        }
        frame.push('\n');
    } 

    let score = format!("Score: {}\n", g.score);
    frame.push_str(&score);
    
    return frame;
}

