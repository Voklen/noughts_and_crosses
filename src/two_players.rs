#[path = "base.rs"]pub mod base;

pub fn main() {
    use base::{Player, CellState, Winner, print_board, player_move, get_winner};
    let mut board: [[CellState; 3]; 3] = [[CellState::None; 3]; 3];
    let mut current_player = Player::Noughts;
    println!("2 Player Game:");
    while get_winner(board) == Winner::StillPlaying {
        print_board(board);
        if current_player == Player::Noughts {
            board = player_move(board, Player::Noughts);
            current_player = Player::Crosses;
        } else {
            board = player_move(board, Player::Crosses);
            current_player = Player::Noughts;
        };
    };
    match get_winner(board) {
        Winner::Crosses=>println!("AND THE WINNER IS: Crosses!"),
        Winner::Noughts=>println!("AND THE WINNER IS: Noughts!"),
        Winner::Draw=>println!("AND ITS A DRAW!"),
        Winner::StillPlaying=>println!("AND THE WINNER IS: wait, nobody?! What. Something's gone VERY wrong...")
    };
}