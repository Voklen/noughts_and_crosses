#[derive(Debug, PartialEq, Eq)]
pub enum Player {
    Noughts,
    Crosses
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellState {
    Noughts,
    Crosses,
    None
}

#[derive(Debug, PartialEq, Eq)]
pub enum Winner {
    Noughts,
    Crosses,
    Draw,
    StillPlaying
}

const ROWS: usize = 3;
const COLLUMNS: usize = 3;

pub fn print_board(board: [[CellState; ROWS]; COLLUMNS]) {
    // THIS BOARD GOES DOWN THEN ALONG (Basicly rows are collums and collums are rows)
    // So a 'X' in the top left would be at 1,3
    // This only affects printing no for everywhere else rows and collumns are still as they are in the array
    let row_length = 3;
    for row in board{
        println!("\n{}-","--".repeat(row_length));
        for i in row {
            print!("|{}", match i{
                    CellState::Crosses=>"X",
                    CellState::Noughts=>"O",
                    CellState::None=>" "
            });
        };
        print!("|");
    };
    println!("\n{}-","--".repeat(row_length));
}

pub fn player_move(mut board: [[CellState; ROWS]; COLLUMNS], player: Player) -> [[CellState; ROWS]; COLLUMNS] {
    println!("{:?}, play a move (1-9)", player);
    let mut move_pos: usize = text_io::read!();
    move_pos -= 1;
    while board[move_pos / COLLUMNS][move_pos % COLLUMNS] != CellState::None {
        println!("That square is already used, try a different move");
        println!("{:?}, play a move (0-8)", player);
        move_pos = text_io::read!();
        move_pos -= 1;
    }
    board[move_pos / COLLUMNS][move_pos % COLLUMNS] = if player == Player::Noughts {CellState::Noughts} else {CellState::Crosses};
    board
}

pub fn get_winner(board: [[CellState; ROWS]; COLLUMNS]) -> Winner{
    // Check collumns
    for collumn in board {
        if collumn == [CellState::Noughts; ROWS]{
           return Winner::Noughts
        } else if collumn == [CellState::Crosses; ROWS] {
            return Winner::Crosses;
        }
    }
    // Check rows
    let mut in_a_row;
    let mut prev_element = CellState::None;
    for row in 0..ROWS {
        in_a_row = true;
        for collumn in board {
            if collumn[row] != prev_element{ // If the elements inthis row are not the same, break
                in_a_row = false;
                break;
            }
            prev_element = collumn[row];
        }
        if in_a_row == true {
            match prev_element {
                CellState::Crosses=>return Winner::Crosses,
                CellState::Noughts=>return Winner::Noughts,
                CellState::None=>()
            }
        }
    }
    for row in board {
        for i in row {
            if i == CellState::None {
                return Winner::StillPlaying;
            }
        }
    }
    return Winner::Draw;
}