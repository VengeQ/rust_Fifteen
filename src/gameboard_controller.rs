use super::Gameboard;
use piston::input::{GenericEvent, Button, MouseButton};

pub struct GameboardController {
    pub gameboard: Gameboard,
    pub selected: Option<[usize; 2]>,
    cursor_pos: [f64; 2],
}

///Maybe some functions should be remove from model to controller
impl GameboardController {
    pub fn new(gameboard: Gameboard) -> Self {
        GameboardController { gameboard, selected: None, cursor_pos: [0_f64;2] }
    }

    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, event: &E) {
        if let Some(pos) = event.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            // Find coordinates relative to upper left corner.
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            // Check that coordinates are inside board boundaries.
            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                // Compute the cell position.
                let cell_x = (x / size * 4.0) as usize;
                let cell_y = (y / size * 4.0) as usize;
                self.selected = Some([cell_x, cell_y]);
                dbg!("x:{} y:{}",cell_x,cell_y);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_gameboard_controller_test_smoke() {
        let _gb = GameboardController::new(Gameboard::new());
    }
}