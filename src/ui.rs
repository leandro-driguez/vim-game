use crate::game::Game;

pub fn get_frame(g: &Game) -> String {
    let mut frame = String::from("");
    for i in 0..(g.height as usize) {
        let mut current_line = String::from("");
        for j in 0..(g.width as usize) {
           current_line = format!("{} {}", current_line, if g.grid[i][j] == 0 { " " } else { if g.grid[i][j] == 1 {"o"} else { "x" } }); 
        }
        frame = format!("{}\n{}", frame, current_line);
    } 
    return frame;
}

