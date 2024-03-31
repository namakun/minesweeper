use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
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
    first_click: bool, // 最初のクリックかどうかを追跡するフラグ
}

#[wasm_bindgen]
impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let cells = vec![vec![Cell { is_open: false, is_mine: false, is_flagged: false }; width]; height];
        Board {
            cells,
            first_click: true, // 初期状態ではtrue
        }
    }

    pub fn get_cell_state(&self, x: usize, y: usize) -> JsValue {
        to_value(&self.cells[y][x]).unwrap()
    }

    // 地雷を配置するメソッドを改良
    pub fn place_mines(&mut self, mines_count: usize, exclude_x: usize, exclude_y: usize) {
        if !self.first_click { // 最初のクリックではない場合は何もしない
            return;
        }

        let mut rng = thread_rng();
        let mut all_positions = (0..self.cells.len() * self.cells[0].len()).collect::<Vec<_>>();
        all_positions.shuffle(&mut rng);

        for &position in all_positions.iter().take(mines_count) {
            let y = position / self.cells[0].len();
            let x = position % self.cells[0].len();

            // 最初にクリックされたセルを除外
            if x == exclude_x && y == exclude_y {
                continue;
            }

            self.cells[y][x].is_mine = true;
        }

        self.first_click = false; // 地雷が配置されたので、フラグをfalseにする
    }

    // セルを開くメソッドを拡張
    pub fn open_cell(&mut self, x: usize, y: usize) -> Vec<ChangedCell> {
        let mut changed_cells = Vec::new();
        if self.first_click {
            self.place_mines_delayed(10, x, y); // 最初のクリックで地雷を配置
            self.first_click = false;
        }

        let cell = &mut self.cells[y][x];
        // 地雷のセルを開いた場合
        if cell.is_mine {
            cell.is_open = true; // このセルを開く
            changed_cells.push(ChangedCell { x, y }); // 変更されたセルをリストに追加
            // ここでゲームオーバーの追加処理が必要になるかもしれません
            return changed_cells; // この時点で処理を終了
        }

        self.recursive_open(x as i32, y as i32, &mut changed_cells);
        changed_cells
    }

    // 再帰的にセルを開くための補助メソッド
    fn recursive_open(&mut self, x: i32, y: i32, changed_cells: &mut Vec<ChangedCell>) {
        if x < 0 || x >= self.cells[0].len() as i32 || y < 0 || y >= self.cells.len() as i32 {
            return; // 範囲外の場合は処理を終了
        }
        let cell = &mut self.cells[y as usize][x as usize];
        if cell.is_open || cell.is_flagged || cell.is_mine {
            return; // 既に開かれている、フラグが立てられている、または地雷がある場合は処理を終了
        }

        cell.is_open = true; // セルを開く
        changed_cells.push(ChangedCell { x: x as usize, y: y as usize }); // 実際に開かれたセルをリストに追加

        // 周囲に地雷がない場合、隣接するセルも開く
        if self.count_mines_around(x as usize, y as usize) == 0 {
            let offsets = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
            for (dx, dy) in offsets.iter() {
                self.recursive_open(x + dx, y + dy, changed_cells); // 再帰的に開く、変更されたセルのリストを渡す
            }
        }
    }


    // 指定されたセルの周囲の地雷数を計算するメソッド
    pub fn count_mines_around(&self, x: usize, y: usize) -> usize {
        let offsets = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        let mut count = 0;
        for (dx, dy) in offsets.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < self.cells[0].len() as i32 && ny >= 0 && ny < self.cells.len() as i32 {
                if self.cells[ny as usize][nx as usize].is_mine {
                    count += 1;
                }
            }
        }
        count
    }

    // 地雷を配置するメソッドを改良
    pub fn place_mines_delayed(&mut self, mines_count: usize, first_x: usize, first_y: usize) {
        if !self.first_click { // 最初のクリックではない場合は何もしない
            return;
        }

        let mut rng = thread_rng();
        let mut all_positions = (0..self.cells.len() * self.cells[0].len()).collect::<Vec<_>>();
        all_positions.shuffle(&mut rng);

        // 最初にクリックされたセルとその周囲8方向を除外するためのリストを作成
        let mut exclude_positions = Vec::new();
        let offsets = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1), (0, 0)];
        for (dx, dy) in offsets.iter() {
            let nx = first_x as i32 + dx;
            let ny = first_y as i32 + dy;
            if nx >= 0 && nx < self.cells[0].len() as i32 && ny >= 0 && ny < self.cells.len() as i32 {
                exclude_positions.push(ny as usize * self.cells[0].len() + nx as usize);
            }
        }

        for &position in all_positions.iter().take(mines_count) {
            if exclude_positions.contains(&position) {
                continue; // 最初にクリックされたセルとその周囲は除外
            }
            let y = position / self.cells[0].len();
            let x = position % self.cells[0].len();
            self.cells[y][x].is_mine = true;
        }

        self.first_click = false; // 地雷が配置されたので、フラグをfalseにする
    }


    // ゲームオーバーの判定（任意のセルが地雷かどうか）
    pub fn is_game_over(&self) -> bool {
        self.cells.iter().flatten().any(|cell| cell.is_open && cell.is_mine)
    }

    // ゲームの勝利条件のチェック
    pub fn check_win(&self) -> bool {
        self.cells.iter().flatten().all(|cell| cell.is_mine || (cell.is_open && !cell.is_mine))
    }

    // セルに旗を立てる/旗を取り除くメソッド
    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        if !self.cells[y][x].is_open {
            self.cells[y][x].is_flagged = !self.cells[y][x].is_flagged;
        }
    }
}
