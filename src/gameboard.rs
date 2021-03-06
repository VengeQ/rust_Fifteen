extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fmt;

pub const SIZE: usize = 4;
pub const FSIZE: f64 = 4.0;

///
/// Current cells with [[u8; SIZE]; SIZE]
/// may be inappropriate, and [u8;SIZE*SIZE]
/// with easy linear arithmetic (web-assembly plain style) looks good.
///
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Gameboard {
    pub cells: [[u8; SIZE]; SIZE],
    pub moves: usize,
}

impl fmt::Display for Gameboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let transposed = self.transpose_flatten();

        let res: Vec<String> = transposed.iter()
            .map(|x| Gameboard::normalize(*x))
            .collect();

        let mut result = "".to_owned();
        for (i, _) in res.iter().enumerate() {
            result += &res[i];
            if (i + 1) % 4 == 0 {
                result += "\n"
            } else {
                result += " ";
            }
        }

        write!(f, "{}", result)
    }
}


///
/// May be some functions should contains in controller?!
///
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
        Gameboard { cells, moves: 0 }
    }

    /// return 0-biased zero field (x,y). Zero has value of 16
    pub fn zero(&self) -> [usize; 2] {
        for x in 0..4 {
            for y in 0..4 {
                if self.cells[x][y] == 16 {
                    return [x, y];
                }
            }
        }
        panic!("No zero value found")
    }

    /// Shuffle values while init new board
    fn shuffle_vec() -> Vec<u8> {
        let mut rng = thread_rng();
        let mut vec: Vec<u8> = (1..=16).collect();
        vec.shuffle(&mut rng);
        vec
    }

    /// Fill empty symbol
    fn normalize(x: u8) -> String {
        match x {
            16 => "  ".to_owned(),
            v if v < 10 => format!("0{}", v),
            v => format!("{}", v)
        }
    }

    pub fn cell_as_string(&self, idx: [usize; 2]) -> String {
        Self::normalize(self.cells[idx[0]][idx[1]])
    }


    ///Checks cells for neighbouring
    pub fn is_neighbours(first: [usize; 2], second: [usize; 2]) -> bool {
        if first == second {
            return false;
        }
        //Соседи всегда на одной линии
        if first[0] != second[0] && first[1] != second[1] {
            return false;
        }

        let (i_first, i_second) =
            ((first[0] as isize, first[1] as isize), (second[0] as isize, second[1] as isize));

        if i_first.0 == i_second.0 && (i_first.1 - i_second.1).abs() == 1 {
            return true;
        }
        if i_first.1 == i_second.1 && (i_first.0 - i_second.0).abs() == 1 {
            return true;
        }

        false
    }

    ///Main action. Swap cells with empty cells
    pub fn swap_with_zero(&mut self, cell: [usize; 2]) -> bool {
        let zero = self.zero();
        // dbg!(cell);
        //  dbg!(zero);
        let isn = Gameboard::is_neighbours(cell, zero);
        //  dbg!("Is_neighbours:{}",isn);
        if isn {
            self.moves += 1;
            let temporary = self.cells[zero[0]][zero[1]];
            self.cells[zero[0]][zero[1]]  = std::mem::replace(& mut self.cells[cell[0]][cell[1]],temporary);

            true
        } else {
            false
        }
    }


    fn transpose_flatten(&self) -> Vec<u8> {
        let mut transposed = self.cells;
        for i in 0..transposed.len() {
            for j in 0..transposed.len() {
                transposed[i][j] = self.cells[j][i];
            }
        }
        transposed.iter().flatten().copied().collect::<Vec<u8>>()
    }

    //Need to implemnt
    pub fn is_over(&self) -> bool {
        fn check_order(vec: &[u8], value: u8) -> bool {
            match vec.first() {
                None => true,
                Some(current) => value < *current && check_order(&vec[1..], *current)
            }
        }
        let v = self.transpose_flatten();
        check_order(&v, 0)
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
        //  dbg!(&gameboard);
        //Каждое число встречается не более одного раза
        for x in &gameboard_flatten {
            assert_eq!(gameboard_flatten.iter().filter(|v| *v == x).count(), 1);
        }
    }

    #[test]
    fn zero_test_smoke() {
        for _ in 0..100 {
            let gameboard = Gameboard::new();
            assert!(gameboard.zero()[0] <= 3);
            assert!(gameboard.zero()[1] <= 3);
        }
    }

    #[test]
    fn is_neighbours_test() {
        let c1 = [2_usize, 3_usize];
        let c2 = [1_usize, 0_usize];
        let c3 = [2_usize, 2_usize];
        let c4 = [1_usize, 3_usize];
        assert_eq!(Gameboard::is_neighbours(c1, c2), false);
        assert_eq!(Gameboard::is_neighbours(c1, c3), true);
        assert_eq!(Gameboard::is_neighbours(c1, c4), true);
        assert_eq!(Gameboard::is_neighbours(c2, c3), false);
        assert_eq!(Gameboard::is_neighbours(c2, c4), false);
        assert_eq!(Gameboard::is_neighbours(c2, c3), false);
    }

    #[test]
    fn swap_with_zero_test() {
        for _ in 0..100 {
            let mut g = Gameboard::new();

            let zero = g.zero();
            if zero == [2, 2] {
                let before = g.cells[2][3];
                println!("before");
                println!("{}", g);
                assert_eq!(g.swap_with_zero([2, 3]), true);
                println!("after");
                println!("{}", g);
                assert_eq!(g.cells[2][2], before);
                assert_eq!(g.cells[2][3], 16);
            }
        }
    }

    #[test]
    fn display_show() {
        let mut g = Gameboard::new();
        println!("{}", g);
    }

    #[test]
    fn sell_as_string_test() {
        for _ in 0..100 {
            let g = Gameboard::new();
            let cs = g.cell_as_string([2, 2]);
            let cu = g.cells[2][2];
            match cu {
                16 => assert_eq!(&cs[..], "  "),
                1..=9 => assert_eq!(&cs, &format!("0{}", cu)),
                _ => assert_eq!(&format!("{}", cu), &cs)
            }
        }
    }

    #[test]
    fn transpose_test() {
        let g = Gameboard::new();
        let tr = g.transpose_flatten();
        let c = g.cells;
        let manual_tr = vec![
            c[0][0], c[1][0], c[2][0], c[3][0],
            c[0][1], c[1][1], c[2][1], c[3][1],
            c[0][2], c[1][2], c[2][2], c[3][2],
            c[0][3], c[1][3], c[2][3], c[3][3]];
        assert_eq!(tr, manual_tr);
    }

    #[test]
    fn is_over_test() {
        for _ in 0..100 {
            let g = Gameboard::new();
            assert_eq!(g.is_over(), false);
        }
        let mut g = Gameboard::new();
        assert_eq!(g.is_over(), false);
        let mut cells = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                cells[j][i] = i as u8 * 4 + j as u8 + 1;
            }
        }
        g.cells = cells;
        println!("{}", g);
        assert_eq!(g.is_over(), true);
    }
}
