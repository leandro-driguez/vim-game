mod game;

fn get_frame(g: game::Game) -> String {
    let mut frame = String::from("");
    for i in 0..g.height {
        let mut current_line = String::from("");
        for j in 0..g.width {
           current_line = format!("{} {}", current_line, if g[i][j] == 0 { " " } else { if g[i][j] == 1 {"o"} else { "x" } }); 
        }
        frame = format!("{}\n{}", frame, current_line);
    } 
    return frame;
}

