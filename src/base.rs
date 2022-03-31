#[derive(Debug, PartialEq, Eq)]
pub enum Player {
	Noughts,
	Crosses,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellState {
	Noughts,
	Crosses,
	None,
}

const ROWS: usize = 3;
const COLUMNS: usize = 3;

pub fn print_board(board: [[CellState; ROWS]; COLUMNS]) {
	// THIS BOARD GOES DOWN THEN ALONG (Basically rows are columns and columns are rows)
	// So a 'X' in the top left would be at 1,3
	// This only affects printing no for everywhere else rows and columns are still as they are in the array
	let row_length = 3;
	for row in board {
		println!("\n{}-", "--".repeat(row_length));
		for i in row {
			print!(
				"|{}",
				match i {
					CellState::Crosses => "X",
					CellState::Noughts => "O",
					CellState::None => " ",
				}
			);
		}
		print!("|");
	}
	println!("\n{}-", "--".repeat(row_length));
}

pub fn player_move(
	mut board: [[CellState; ROWS]; COLUMNS],
	player: Player,
) -> [[CellState; ROWS]; COLUMNS] {
	println!("{:?}, play a move (1-9)", player);
	let mut move_pos: usize = text_io::read!();
	move_pos -= 1;
	while board[move_pos / COLUMNS][move_pos % COLUMNS] != CellState::None {
		println!("That square is already used, try a different move");
		println!("{:?}, play a move (0-8)", player);
		move_pos = text_io::read!();
		move_pos -= 1;
	}
	board[move_pos / COLUMNS][move_pos % COLUMNS] = if player == Player::Noughts {
		CellState::Noughts
	} else {
		CellState::Crosses
	};
	board
}

pub fn get_winner(board: [[CellState; ROWS]; COLUMNS]) -> Option<CellState> {
	// Check columns
	for column in board {
		if column == [CellState::Noughts; ROWS] {
			return Some(CellState::Noughts);
		} else if column == [CellState::Crosses; ROWS] {
			return Some(CellState::Crosses);
		}
	}
	// Check rows
	let mut in_a_row;
	let mut prev_element;
	for row in 0..ROWS {
		in_a_row = true;
		prev_element = board[0][row];
		for column in board {
			if column[row] != prev_element {
				// If the elements in this row are not the same, break
				in_a_row = false;
				break;
			}
			prev_element = column[row];
		}
		if in_a_row == true && prev_element != CellState::None {
			// Only true if all columns have been looped through and all elements match
			return Some(prev_element);
		}
	}
	// Check diagonal from 0,0
	in_a_row = true; // Can reuse mutable variable from row check as it is no longer needed
	let max_diagonal = if ROWS > COLUMNS { COLUMNS } else { ROWS };
	for i in 1..max_diagonal {
		if (board[i][i]) != board[i - 1][i - 1] {
			in_a_row = false;
			break;
		}
	}
	if in_a_row && board[0][0] != CellState::None {
		// Only true if all columns have been looped through and all elements match
		return Some(board[0][0]);
	}
	// Check other diagonal
	in_a_row = true; // Can reuse mutable variable from row check as it is no longer needed
	for i in 1..max_diagonal {
		if (board[i][COLUMNS - 1 - i]) != board[i - 1][COLUMNS - i] {
			in_a_row = false;
			break;
		}
	}
	if in_a_row && board[0][COLUMNS - 1] != CellState::None {
		// Only true if all columns have been looped through and all elements match
		return Some(board[0][COLUMNS - 1]);
	}
	// If nobody's won, check if the game is still going
	for row in board {
		for i in row {
			if i == CellState::None {
				return None; // If there is still a free space on the board and nobody's won, the game is still going
			}
		}
	}
	// If nobody's won and there are no free spaces, it's a draw.
	return Some(CellState::None);
}
