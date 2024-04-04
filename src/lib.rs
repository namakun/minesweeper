use wasm_bindgen::prelude::*;
use rand::thread_rng;
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Cell {
    pub is_open: bool,
    pub is_mine: bool,
    pub is_flagged: bool,
}

#[wasm_bindgen]
pub struct ChangedCell {
    pub x: usize,
    pub y: usize,
}

#[wasm_bindgen]
pub struct Board {
    cells: Vec<Vec<Cell>>,
    first_click: bool,
}

#[wasm_bindgen]
impl Board {
    // Boardインスタンスを作成します。
    pub fn new(width: usize, height: usize) -> Board {
        let cells = vec![vec![Cell { is_open: false, is_mine: false, is_flagged: false }; width]; height];
        Board {
            cells,
            first_click: true,
        }
    }

    // 指定されたセルを開きます。最初のクリック時には地雷を配置します。
    pub fn open_cell(&mut self, x: usize, y: usize, mines_count: usize) -> Vec<ChangedCell> {
        let mut changed_cells = Vec::new();
        if self.first_click {
            self.place_mines(mines_count, x, y);
            self.first_click = false;
        }

        if self.can_open_cell(x, y) {
            let cell = &mut self.cells[y][x];
            if cell.is_mine {
                cell.is_open = true;
                changed_cells.push(ChangedCell { x, y });
                return changed_cells;
            }
            self.recursive_cell_open(x as i32, y as i32, &mut changed_cells);
        }
        changed_cells
    }

    // セルに旗を立てるか、旗を取り除きます。
    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        if !self.cells[y][x].is_open {
            self.cells[y][x].is_flagged = !self.cells[y][x].is_flagged;
        }
    }

    // ゲームが終了したか判断します。
    pub fn is_game_over(&self) -> bool {
        self.cells.iter().flatten().any(|cell| cell.is_open && cell.is_mine)
    }

    // ゲームをクリアしたか判断します。
    pub fn is_game_clear(&self) -> bool {
        self.cells.iter().flatten().all(|cell| cell.is_mine || (cell.is_open && !cell.is_mine))
    }

    // 指定されたセルの状態を取得します。
    pub fn get_cell_state(&self, x: usize, y: usize) -> JsValue {
        to_value(&self.cells[y][x]).unwrap()
    }

    // 指定されたセルの周囲にある地雷の数を数えます。
    pub fn count_mines_around(&self, x: usize, y: usize) -> usize {
        let width = self.cells[0].len() as i32;
        let height = self.cells.len() as i32;
        let mut count = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < width && ny >= 0 && ny < height && self.cells[ny as usize][nx as usize].is_mine {
                    count += 1;
                }
            }
        }

        count
    }

    // 地雷を配置します。
    fn place_mines(&mut self, mines_count: usize, first_x: usize, first_y: usize) {
        if !self.first_click || mines_count >= self.cells.len() * self.cells[0].len() {
            return;
        }

        let mut rng = thread_rng();
        let mut placed_mines = 0;

        while placed_mines < mines_count {
            let x = rng.gen_range(0..self.cells[0].len());
            let y = rng.gen_range(0..self.cells.len());

            if ((x as i32 - first_x as i32).abs() <= 1 && (y as i32 - first_y as i32).abs() <= 1) || self.cells[y][x].is_mine {
                continue;
            }

            self.cells[y][x].is_mine = true;
            placed_mines += 1;
        }

        self.first_click = false;
    }

    // 周辺のセルを確認し、地雷のないセルを再帰的に開きます。
    fn recursive_cell_open(&mut self, x: i32, y: i32, changed_cells: &mut Vec<ChangedCell>) {
        if x < 0 || x >= self.cells[0].len() as i32 || y < 0 || y >= self.cells.len() as i32 || !self.can_open_cell(x as usize, y as usize) {
            return;
        }

        let cell = &mut self.cells[y as usize][x as usize];
        cell.is_open = true;
        changed_cells.push(ChangedCell { x: x as usize, y: y as usize });

        if self.count_mines_around(x as usize, y as usize) == 0 {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    self.recursive_cell_open(x + dx, y + dy, changed_cells);
                }
            }
        }
    }

    // 指定されたセルを開けるか判断します。
    fn can_open_cell(&self, x: usize, y: usize) -> bool {
        x < self.cells[0].len() && y < self.cells.len() &&
        !self.cells[y][x].is_open && !self.cells[y][x].is_flagged
    }
}
