extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod gameboard;
mod gameboard_controller;



pub use gameboard::Gameboard;
pub use gameboard_controller::GameboardController;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};


fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", [512; 2])
        .graphics_api(opengl)
        .exit_on_esc(true);
    let mut window: Window = settings.build()
        .expect("Could not create window");
    let mut events = Events::new(EventSettings::new().lazy(true)); //lazy, так как анимации не будет никакой.
    let mut gl = GlGraphics::new(opengl);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;
                clear([1.0; 4], g);
            });
        }
    }
}