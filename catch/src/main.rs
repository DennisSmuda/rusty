extern crate piston_window;

use piston_window::*;

use piston_window::PistonWindow;
use piston_window::Texture;
use piston_window::EventLoop;

struct Rect {
    pub color: [f32; 4],
    pub position: [f64; 4]
}

impl Rect {
    fn new() -> Self {
        Rect {
            color: [1.0, 0.5, 0.25, 1.0],
            position: [0.0, 0.0, 100.0, 100.0],
        }
    }
}



fn main() {
    let mut rect = Rect::new();

    // Create new Window
    let mut window: PistonWindow = WindowSettings::new("Hello World", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Setup Event Loop
    while let Some(e) = window.next() {
        match e {
            Input::Render(r) => {
                window.draw_2d(&e, |c, g| {
                    clear([0.5, 1.0, 0.5, 1.0], g);
                });
            }
            Input::Press(b) => {
                match b {
                    Button::Keyboard(k) => {
                        match k {
                            Key::W => {
                                println!("Press W");
                            }
                            _ => {}
                        }
                    }
                    _ => {} // Ignore Buttons
                }
            }
            _ => {} // Ignore rest of events
        }
    }
}
