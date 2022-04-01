#[path = "base.rs"]
pub mod base;

use base::{get_winner, player_move, print_board, CellState, Player};

pub fn main() {
	println!("2 Player Game:");
	let board: [[CellState; 3]; 3] = [[CellState::None; 3]; 3];
	let current_player = Player::Noughts;
	recursive_game_loop(board, current_player)
}

fn recursive_game_loop(mut board: [[CellState; 3]; 3], player: Player) {
	print_board(board);
	board = player_move(board, player.to_cellstate());

	match get_winner(board) {
		Some(CellState::Crosses) => println!("AND THE WINNER IS: Crosses!"),
		Some(CellState::Noughts) => println!("AND THE WINNER IS: Noughts!"),
		Some(CellState::None) => println!("AND ITS A DRAW!"),
		None => recursive_game_loop(board, player.alternate()),
	};
}
