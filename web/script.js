import init, { Board } from "./minesweeper.js";

// 定数と設定
const GAME_SETTINGS = {
  width: 8,
  height: 15,
  minesCount: 15,
};

// ゲームの状態
let board;
let gameOver = false;

// 初期化関数
async function run() {
  await init();
  restartGame();
}

// UIの初期化とイベントリスナーの設定
function setupUI() {
  document.getElementById("restartButton").addEventListener("click", restartGame);
  updateUI();
}

// ゲームの再開始
function restartGame() {
  board = Board.new(GAME_SETTINGS.width, GAME_SETTINGS.height);
  gameOver = false;
  setupUI();
  document.getElementById("gameOver").style.visibility = "hidden";
  document.getElementById("gameClear").style.visibility = "hidden";
}

// UIの更新
function updateUI() {
  const gameContainer = document.getElementById("game");
  gameContainer.innerHTML = "";
  for (let y = 0; y < GAME_SETTINGS.height; y++) {
    const rowDiv = document.createElement("div");
    rowDiv.style.display = "flex";
    for (let x = 0; x < GAME_SETTINGS.width; x++) {
      rowDiv.appendChild(createCellElement(x, y));
    }
    gameContainer.appendChild(rowDiv);
  }
}

// セル要素の作成
function createCellElement(x, y) {
  const cellDiv = document.createElement("div");
  cellDiv.classList.add("cell");
  cellDiv.addEventListener("click", (e) => {
    e.preventDefault();
    if (gameOver || board.get_cell_state(x, y).is_flagged) return;
    handleCellClick(x, y);
  });
  cellDiv.addEventListener("contextmenu", (e) => {
    e.preventDefault();
    toggleFlag(x, y, cellDiv);
  });
  return cellDiv;
}

// セルのクリック処理
function handleCellClick(x, y) {
  const changedCells = board.open_cell(x, y, GAME_SETTINGS.minesCount);
  changedCells.forEach(({ x, y }) => {
    const minesAround = board.count_mines_around(x, y);
    updateCellUI(x, y, minesAround);
  });
  checkGameState();
}

// 旗の切り替え
function toggleFlag(x, y, cellDiv) {
  if (gameOver) return;
  board.toggle_flag(x, y);
  handleFlagToggle(cellDiv, board.get_cell_state(x, y));
}

// ゲーム状態の確認
function checkGameState() {
  if (board.is_game_over()) {
    gameOver = true;
    revealMines();
    document.getElementById("gameOver").style.visibility = "visible";
  } else if (board.is_game_clear()) {
    document.getElementById("gameClear").style.visibility = "visible";
  }
}

// セルUIの更新
function updateCellUI(x, y, minesAround) {
  const cellState = board.get_cell_state(x, y);
  const cellDiv = document.querySelector(`#game div:nth-child(${y + 1}) div:nth-child(${x + 1})`);
  cellDiv.className = 'cell';
  if (cellState.is_open) {
    cellDiv.classList.add('cell-open');
    cellDiv.textContent = minesAround > 0 ? minesAround : "";
    if (minesAround > 0) {
      cellDiv.classList.add(`cell-${minesAround}`);
    }
  } else if (cellState.is_flagged) {
    cellDiv.textContent = "🚩";
    cellDiv.classList.add('cell-flagged');
  } else {
    cellDiv.textContent = "";
  }
}

// 旗のUI更新
function handleFlagToggle(cellDiv, cellState) {
  cellDiv.textContent = cellState.is_flagged ? "🚩" : "";
}

// 地雷の公開
function revealMines() {
  for (let y = 0; y < GAME_SETTINGS.height; y++) {
    for (let x = 0; x < GAME_SETTINGS.width; x++) {
      const cellState = board.get_cell_state(x, y);
      if (cellState.is_mine) {
        const cellDiv = document.querySelector(`#game div:nth-child(${y + 1}) div:nth-child(${x + 1})`);
        cellDiv.textContent = "💣";
        cellDiv.classList.add('cell-open');
      }
    }
  }
}

run();
