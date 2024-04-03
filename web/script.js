import init, { Board } from "./minesweeper.js";

let board;
let firstClick = true;
let gameOver = false;

document.getElementById("restartButton").addEventListener("click", restartGame);

async function run() {
  await init();
  restartGame();
}

function restartGame() {
  board = Board.new(9, 9);
  firstClick = true;
  gameOver = false;
  updateUI(board);
  const gameOverMessage = document.getElementById("gameOver");
  gameOverMessage.style.visibility = "hidden";
  const gameClearMessage = document.getElementById("gameClear");
  gameClearMessage.style.visibility = "hidden";
}

function updateUI(board) {
  const gameContainer = document.getElementById("game");
  gameContainer.innerHTML = "";

  for (let y = 0; y < 9; y++) {
    const rowDiv = document.createElement("div");
    rowDiv.style.display = "flex";

    for (let x = 0; x < 9; x++) {
      const cellDiv = document.createElement("div");
      cellDiv.classList.add("cell");

      cellDiv.addEventListener("click", (e) => {
        e.preventDefault();
        if (gameOver || board.get_cell_state(x, y).is_flagged) return;
        if (!board.get_cell_state(x, y).is_open) {
          handleCellClick(board, x, y, cellDiv);
        }
      });

      cellDiv.addEventListener("contextmenu", (e) => {
        e.preventDefault();
        if (gameOver) return;
        if (!board.get_cell_state(x, y).is_open) {
          board.toggle_flag(x, y);
          handleFlagToggle(cellDiv, board.get_cell_state(x, y));
        }
      });

      rowDiv.appendChild(cellDiv);
    }
    gameContainer.appendChild(rowDiv);
  }
}
function updateCellUI(x, y) {
  const cellState = board.get_cell_state(x, y);
  const cellDiv = document.querySelector(`#game div:nth-child(${y + 1}) div:nth-child(${x + 1})`);

  if (cellState.is_open) {
    const minesAround = board.count_mines_around(x, y);
    cellDiv.textContent = minesAround > 0 ? minesAround : "";
    cellDiv.style.backgroundColor = "#bfbfbf";

    switch (minesAround) {
      case 1:
        cellDiv.style.color = "blue";
        break;
      case 2:
        cellDiv.style.color = "green";
        break;
      case 3:
        cellDiv.style.color = "red";
        break;
      case 4:
        cellDiv.style.color = "purple";
        break;
      default:
        cellDiv.style.color = "black";
    }
  }

  // 旗が立っている場合の処理もここに追加
  // ...
}

function handleFlagToggle(cellDiv, cellState) {
  if (cellState.is_flagged) {
    cellDiv.textContent = "🚩"; // 旗を表示
  } else {
    cellDiv.textContent = cellState.is_open ? cellState.mines_around.toString() : "";
  }
}

function handleCellClick(board, x, y, cellDiv) {
  const changedCells = board.open_cell(x, y);
  changedCells.forEach((cell) => {
    updateCellUI(cell.x, cell.y);
  });

  if (board.is_game_over()) {
    gameOver = true;
    revealMines(board);
    const gameOverMessage = document.getElementById("gameOver");
    gameOverMessage.style.visibility = "visible";
  }

  if (!gameOver && board.check_win()) {
    const gameClearMessage = document.getElementById("gameClear");
    gameClearMessage.style.visibility = "visible";
  }
}

function revealMines(board) {
  for (let y = 0; y < 9; y++) {
    for (let x = 0; x < 9; x++) {
      const cell = document.querySelector(`#game div:nth-child(${y + 1}) div:nth-child(${x + 1})`);
      if (board.get_cell_state(x, y).is_mine) {
        cell.textContent = "💣";
      }
    }
  }
}

run();
