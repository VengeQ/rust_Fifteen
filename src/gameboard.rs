extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;

const SIZE: usize = 4;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Gameboard {
    pub cells: [[u8; SIZE]; SIZE]
}

impl Gameboard {
    ///
    /// Generate new gameboard with shuffled numbers
    ///
    pub fn new() -> Self {
        let vec = Gameboard::shuffle_vec();
        let mut cells = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                cells[i][j] = vec[i * SIZE + j];
            }
        }
        dbg!(&cells);
        Gameboard { cells }
    }

    pub fn zero(&self) -> (usize, usize) {
        for x in 0..4 {
            for y in 0..4 {
                if self.cells[x][y] == 0 {
                    dbg!("x:{} y:{}",x,y);
                    return (x, y);
                }
            }
        }
        panic!("No null value found")
    }

    fn shuffle_vec() -> Vec<u8> {
        let mut rng = thread_rng();
        let mut vec: Vec<u8> = (0..16).collect();
        vec.shuffle(&mut rng);
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuffle_in_new_test() {
        assert_eq!(Gameboard::shuffle_vec().len(), 16)
    }

    #[test]
    fn new_test() {
        let gameboard = Gameboard::new();
        let gameboard_flatten = gameboard.cells.iter()
            .flat_map(|x| vec![x[0], x[1], x[2], x[3]]).collect::<Vec<u8>>();
        dbg!(&gameboard);
        //Каждое число встречается не более одного раза
        for x in &gameboard_flatten {
            assert_eq!(gameboard_flatten.iter().filter(|v| *v == x).count(), 1);
        }
    }
}
