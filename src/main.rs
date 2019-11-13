//! Use MVC pattern for this APP
//! Model -> gameboard
//! View -> gameboard_view
//! Controller -> gameboard_controller
//!

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod gameboard;
mod gameboard_controller;
mod gameboard_view;

pub use gameboard::Gameboard;
pub use gameboard_controller::GameboardController;
pub use gameboard_view::{GameboardView, GameboardViewSettings};

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings};

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Fifteen", [460, 500])
        .graphics_api(opengl)
        .resizable(false)
        .exit_on_esc(true);
    let mut window: Window = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new()); //lazy, так как анимации не будет никакой.
    let mut gl = GlGraphics::new(opengl);
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/amazone.ttf", (), texture_settings).expect("Could not load font");
    let gameboard = Gameboard::new();
    println!("{}", &gameboard);
    dbg!(&gameboard);
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);
    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(gameboard_view.settings.position, gameboard_view.settings.size, &e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, glyphs, &c, g);
            });
        }
    }
}