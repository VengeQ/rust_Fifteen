use graphics::types::Color;
use graphics::{Context, Graphics, Image, Transformed, text};
use graphics::character::CharacterCache;
use super::gameboard_controller::GameboardController;

///Rendering settings
pub struct GameboardViewSettings {
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
    ///Color of zero element
    pub zero_color: Color,
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
            position: [10.0; 2],
            size: 400.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            zero_color: [1.0, 1.0, 0.5, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            between_color: [0.5, 0.5, 0.5, 1.0],
            board_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            text_color: [1.0, 1.0, 1.0, 1.0],
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
    pub fn draw<G: Graphics, C: CharacterCache<Texture=G::Texture>>(&self, controller: &GameboardController, glyphs: &mut C, c: &Context, g: &mut G) {
        use graphics::{Line, Rectangle};

        let ref settings = self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];

        Rectangle::new(settings.background_color)
            .draw(board_rect, &c.draw_state, c.transform, g);

        let white= [1.0, 1.0, 1.0, 1.0];

        let (zx, zy) = controller.gameboard.zero();
        //dbg!("zx:{} zy:{}",zx,zy);
        let zero_rect = [
            settings.position[0] + settings.size / 4.0 * zx as f64,
            settings.position[1] + settings.size / 4.0 * zy as f64,
            settings.size / 4.0, settings.size / 4.0,
        ];

        if let Some(sel) = controller.selected {
            let selected_rect = [
                settings.position[0] + settings.size / 4.0 * sel[0] as f64,
                settings.position[1] + settings.size / 4.0 * sel[1] as f64,
                settings.size / 4.0, settings.size / 4.0,
            ];

            Rectangle::new(settings.zero_color)
                .draw(selected_rect, &c.draw_state, c.transform, g);
        }


        Rectangle::new(white)
            .draw(zero_rect, &c.draw_state, c.transform, g);


        let cell_edge = Line::new(settings.between_color, settings.cell_edge_radius);

        for i in 0..4 {
            let x = settings.position[0] + i as f64 / 4.0 * settings.size;
            let y = settings.position[1] + i as f64 / 4.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            cell_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);
        }
        Rectangle::new_border(settings.border_color, settings.board_edge_radius)
            .draw(board_rect, &c.draw_state, c.transform, g);

        //Draw cells charactes
        let text_image = Image::new_color(settings.text_color);
        let cell_size = settings.size / 4.0;
        for j in 0..4 {
            for i in 0..4 {
                let ch = controller.gameboard.cell_as_string((i, j));
                let pos = [
                    settings.position[0] + i as f64 * cell_size + 30.0,
                    settings.position[1] + j as f64 * cell_size + 60.0
                ];

                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 40)
                    .draw(&ch,
                          glyphs,
                          &c.draw_state,
                          c.transform.trans(pos[0], pos[1]),
                          g).unwrap_or_else(|_|());
            }
        }
    }
}




