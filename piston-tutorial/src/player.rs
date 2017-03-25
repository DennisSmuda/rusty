
pub struct Player {
    name: "Player",
    pos: [f64, ..2],
}

impl Player {
    pub fn render(&mut self, args: &RenderArgs) {
        println!("Player Render")
    }
}

