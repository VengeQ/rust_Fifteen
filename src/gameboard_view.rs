use graphics::{Context, Graphics, Image, text, Transformed};
use graphics::character::CharacterCache;
use graphics::types::Color;
use graphics::{Line, Rectangle};
use crate::gameboard_controller::GameState;

use super::gameboard::{FSIZE, SIZE};
use super::gameboard_controller::GameboardController;
use crate::Animator;
use crate::animator::Direction;

///Rendering settings
pub struct GameboardViewSettings {
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
    ///Color of zero element
    pub zero_color: Color,
    ///Color of selected element
    pub selected_color: Color,
    /// Border color.
    pub border_color: Color,
    ///Color between cells;
    pub between_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Edge radius between cells.
    pub cell_edge_radius: f64,
    pub text_color: Color,
}

impl GameboardViewSettings {
    pub fn new() -> Self {
        GameboardViewSettings {
            position: [10.0, 50.0],
            size: 440.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            zero_color: [1.0, 1.0, 1.0, 1.0],
            selected_color: [1.0, 1.0, 0.5, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            between_color: [0.5, 0.5, 0.5, 1.0],
            board_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            text_color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

pub struct GameboardView {
    pub settings: GameboardViewSettings
}

impl GameboardView {
    pub fn new(settings: GameboardViewSettings) -> Self {
        GameboardView { settings }
    }

    /// Draw gameboard.
    pub fn draw<G: Graphics, C: CharacterCache<Texture=G::Texture>>(&self, controller: &mut GameboardController, glyphs: &mut C, c: &Context, g: &mut G) {
        match controller.game_state {
            GameState::Prepare => { self.draw_prepare(controller, glyphs, c, g) }
            GameState::InProcess => { self.draw_progress(controller, glyphs, c, g) }
            GameState::GameOver => {}
        };
    }

    /// Draw prepare
    fn draw_prepare<G: Graphics, C: CharacterCache<Texture=G::Texture>>(&self, _controller: &GameboardController, glyphs: &mut C, c: &Context, g: &mut G) {
        use graphics::Rectangle;
        let ref settings = self.settings;

        //Board
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];
        Rectangle::new(settings.background_color)
            .draw(board_rect, &c.draw_state, c.transform, g);

        //start Game
        let points = format!("Press Space to start!");
        text::Text::new_color(settings.text_color, 40)
            .draw(&points,
                  glyphs,
                  &c.draw_state,
                  c.transform.trans(10.0, settings.size / FSIZE * 1.5),
                  g).unwrap_or_else(|_| ());
    }

    ///Draw in progress
    fn draw_progress<G: Graphics, C: CharacterCache<Texture=G::Texture>>(&self, controller: &mut GameboardController, glyphs: &mut C, c: &Context, g: &mut G) {
        let ref settings = self.settings;

        self.draw_board(c, g);
        self.draw_fields(controller, c, g);
        self.draw_lines(c, g);
        let animate_shift = controller.animator.animate(controller.animate_direction);
        //Draw points
        let points = format!("Moves: {}", controller.gameboard.moves.to_string());
        text::Text::new_color(settings.text_color, 40)
            .draw(&points,
                  glyphs,
                  &c.draw_state,
                  c.transform.trans(settings.size / FSIZE * 1.5, settings.position[1] - 10.0),
                  g).unwrap_or_else(|_| ());

        //Draw cells characters
        let cell_size = settings.size / FSIZE;
        for j in 0..SIZE {
            for i in 0..SIZE {
                let ch = controller.gameboard.cell_as_string([i, j]);
                let pos = [
                    settings.position[0] + i as f64 * cell_size + 30.0,
                    settings.position[1] + j as f64 * cell_size + 70.0
                ];

                let is_over = controller.animator.is_over();
                let tupled = (animate_shift[0], animate_shift[1]);
                let transform = match tupled {
                    (x, _)  if x < 0.0 => { c.transform.trans(pos[0] + 100.0 + animate_shift[0], pos[1]) }
                    (x, _)  if x > 0.0 => { c.transform.trans(pos[0] - 100.0 + animate_shift[0], pos[1]) }
                    (_, y)  if y < 0.0 => { c.transform.trans(pos[0], pos[1] + animate_shift[1] + 100.0) }
                    (_, y)  if y > 0.0 => { c.transform.trans(pos[0], pos[1] + animate_shift[1] - 100.0) }
                    _ => { c.transform.trans(pos[0], pos[1]) }
                };
                if controller.animate_cell == [i, j] {
                    text::Text::new_color(settings.text_color, 40)
                        .draw(&ch,
                              glyphs,
                              &c.draw_state,
                              if is_over {
                                  c.transform.trans(pos[0], pos[1])
                              } else { transform },
                              g).unwrap_or_else(|_| ());
                } else {
                    text::Text::new_color(settings.text_color, 40)
                        .draw(&ch,
                              glyphs,
                              &c.draw_state,
                              c.transform.trans(pos[0], pos[1]),
                              g).unwrap_or_else(|_| ());
                }
            }
        }
    }

    fn draw_board<G: Graphics>(&self, c: &Context, g: &mut G) {
        let settings = &self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];

        Rectangle::new(settings.background_color)
            .draw(board_rect, &c.draw_state, c.transform, g);
        Rectangle::new_border(settings.border_color, settings.board_edge_radius)
            .draw(board_rect, &c.draw_state, c.transform, g);
    }
    fn draw_fields<G: Graphics>(&self, controller: &mut GameboardController, c: &Context, g: &mut G) {
        let settings = &self.settings;
        let animate_shift = controller.animator.animate(controller.animate_direction);
        dbg!(animate_shift);
        //Zero rectangle
        let zx = controller.gameboard.zero()[0];
        let zy = controller.gameboard.zero()[1];
        if controller.animator.is_over() {
            let zero_rect = [
                settings.position[0] + settings.size / FSIZE * zx as f64 ,
                settings.position[1] + settings.size / FSIZE * zy as f64 ,
                settings.size / FSIZE - animate_shift[0], settings.size / FSIZE - animate_shift[1],
            ];
            Rectangle::new(settings.zero_color)
                .draw(zero_rect, &c.draw_state, c.transform, g);
        }
        else {
            let new_zero_rect = [
                settings.position[0] + settings.size / FSIZE * zx as f64 + animate_shift[0],
                settings.position[1] + settings.size / FSIZE * zy as f64 + animate_shift[1],
                settings.size / FSIZE - animate_shift[0], settings.size / FSIZE - animate_shift[1],
            ];
        }


        //Selected rectangle
        if let Some(sel) = controller.selected {
            let selected_rect = [
                settings.position[0] + settings.size / FSIZE * sel[0] as f64,
                settings.position[1] + settings.size / FSIZE * sel[1] as f64,
                settings.size / FSIZE, settings.size / FSIZE,
            ];

            Rectangle::new(settings.selected_color)
                .draw(selected_rect, &c.draw_state, c.transform, g);
        }
    }
    fn draw_lines<G: Graphics>(&self, c: &Context, g: &mut G) {
        let settings = &self.settings;
        let cell_edge = Line::new(settings.between_color, settings.cell_edge_radius);

        for i in 0..4 {
            let x = settings.position[0] + i as f64 / FSIZE * settings.size;
            let y = settings.position[1] + i as f64 / FSIZE * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            cell_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);
        }
    }
}




