use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub is_open: bool,
    pub is_mine: bool,
}

#[wasm_bindgen]
pub struct Board {
    cells: Vec<Vec<Cell>>,
}

#[wasm_bindgen]
impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let cells = vec![vec![Cell { is_open: false, is_mine: false }; width]; height];
        Board { cells }
    }

    pub fn display(&self) -> String {
        let mut display_string = String::new();
        for row in &self.cells {
            for cell in row {
                let symbol = if cell.is_mine { "💣" } else { "⬜" };
                display_string.push_str(&format!("{} ", symbol));
            }
            display_string.push('\n');
        }
        display_string
    }
}

fn main() {
    let board = Board::new(9, 9);
    board.display();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_creation() {
        let width = 9;
        let height = 9;
        let board = Board::new(width, height);

        // ボードのサイズをチェック
        assert_eq!(board.cells.len(), height);
        for row in &board.cells {
            assert_eq!(row.len(), width);
        }

        // すべてのセルが正しく初期化されているかチェック
        for row in &board.cells {
            for cell in row {
                assert_eq!(cell.is_open, false);
                assert_eq!(cell.is_mine, false);
            }
        }
    }
}
