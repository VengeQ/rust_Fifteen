use super::Gameboard;
use piston::input::{GenericEvent, Button, MouseButton, Key};
use crate::gameboard_controller::GameState::{GameOver, InProcess};

pub struct GameboardController {
    pub gameboard: Gameboard,
    pub selected: Option<[usize; 2]>,
    pub game_state: GameState,
    cursor_pos: [f64; 2],
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum GameState {
    Prepare,
    InProcess,
    GameOver,
}

///Maybe some functions should be remove from model to controller
impl GameboardController {
    pub fn new(gameboard: Gameboard) -> Self {
        GameboardController { gameboard, selected: None, game_state: GameState::Prepare, cursor_pos: [0_f64; 2] }
    }

    ///Main function. Swap two neighbour cells, if one is zero cell.
    fn swap_rectangle_or_cancel(&mut self, cell: [usize; 2], prev_cell: [usize; 2]) {
        if self.gameboard.zero() == cell {
            let was_swapped = self.gameboard.swap_with_zero(prev_cell);
            if self.gameboard.is_over(){
                self.game_state=GameOver;
            }
            dbg!(was_swapped);
        }
        println!("moves: {}", self.gameboard.moves);
        println!("{}", self.gameboard);
        self.selected = None;

    }

    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, event: &E) {
        if let Some(pos) = event.mouse_cursor_args() {
            //dbg!(pos);
            self.cursor_pos = pos;
        }

        match self.game_state {
            GameState::Prepare => {
                self.event_prepare(pos, size, event)
            }
            GameState::InProcess => {
                self.event_progress(pos, size, event);

            }
            GameState::GameOver => { self.event_progress(pos, size, event) }
        }
    }

    //event-handler in progress game
    fn event_progress<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, event: &E) {
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            // Find coordinates relative to upper left corner.
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            // Check that coordinates are inside board boundaries.
            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                // Compute the cell position.
                let cell_x = (x / size * 4.0) as usize;
                let cell_y = (y / size * 4.0) as usize;
                //dbg!("x:{} y:{}",cell_x,cell_y);
                match self.selected {
                    Some(sel) => {
                        self.swap_rectangle_or_cancel([cell_x, cell_y], [sel[0], sel[1]]);
                    }
                    None => {
                        if self.gameboard.zero() != [cell_x, cell_y] {
                            self.selected = Some([cell_x, cell_y])
                        }
                    }
                }
            }
        }
        if let Some(Button::Mouse(MouseButton::Right)) = event.press_args() {
            //Unselect
            self.selected = None;
        }
    }

    //event-handler in prepare
    fn event_prepare<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, event: &E) {
        if let Some(Button::Keyboard(Key::Space)) = event.press_args() {
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            // Check that coordinates are inside board boundaries.
            if x >= 0.0 && x < size && y >= 0.0 && y < size {}
            self.game_state = InProcess;
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