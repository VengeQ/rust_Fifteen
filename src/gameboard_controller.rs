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
        GameboardController { gameboard, selected: None, cursor_pos: [0_f64; 2] }
    }

    ///Main function. Swap two neighbour cells, if one is zero cell.
    fn swap_rectangle_or_cancel(&mut self, cell: (usize, usize)) {
        let was_swapped = self.gameboard.swap_with_zero(cell);
        dbg!(was_swapped);
        self.selected = None;
    }


    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, event: &E) {
        if let Some(pos) = event.mouse_cursor_args() {
            //dbg!(pos);
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
                dbg!("x:{} y:{}",cell_x,cell_y);
                match self.selected {
                    Some(sel) => {
                            self.swap_rectangle_or_cancel((sel[0],sel[1]));
                    }
                    None => self.selected =Some( [cell_x,cell_y])
                }
            }
        }
        if let Some(Button::Mouse(MouseButton::Right)) = event.press_args() {
            //Unselect
            self.selected = None;
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