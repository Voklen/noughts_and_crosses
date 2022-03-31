#[path = "base.rs"]
pub mod base;

pub fn main() {
	use base::{get_winner, player_move, print_board, CellState, Player};
	let mut board: [[CellState; 3]; 3] = [[CellState::None; 3]; 3];
	let mut current_player = Player::Noughts;
	println!("2 Player Game:");
	while get_winner(board) == None {
		print_board(board);
		if current_player == Player::Noughts {
			board = player_move(board, Player::Noughts);
			current_player = Player::Crosses;
		} else {
			board = player_move(board, Player::Crosses);
			current_player = Player::Noughts;
		};
	}
	match get_winner(board) {
		Some(CellState::Crosses) => println!("AND THE WINNER IS: Crosses!"),
		Some(CellState::Noughts) => println!("AND THE WINNER IS: Noughts!"),
		Some(CellState::None) => println!("AND ITS A DRAW!"),
		None => println!("AND THE WINNER IS: wait, nobody?! What. Something's gone VERY wrong..."),
	};
}
