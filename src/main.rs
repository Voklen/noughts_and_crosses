extern crate text_io;

mod one_player;
pub mod two_players;
fn main() {
    // two_players::main();
    // return;
    println!("Welcome to Alex's:");
    println!("
     /££   /££                               /££         /££                     /£££            /££££££                                                            
    | £££ | ££                              | ££        | ££                    /££ ££          /££__  ££                                                           
    | ££££| ££  /££££££  /££   /££  /££££££ | £££££££  /££££££   /£££££££      |  £££          | ££  |__/  /££££££   /££££££   /£££££££ /£££££££  /££££££   /£££££££
    | ££ ££ ££ /££__  ££| ££  | ££ /££__  ££| ££__  ££|_  ££_/  /££_____/       /££ ££/££      | ££       /££__  ££ /££__  ££ /££_____//££_____/ /££__  ££ /££_____/
    | ££  ££££| ££  | ££| ££  | ££| ££  | ££| ££  | ££  | ££   |  ££££££       | ££  ££_/      | ££      | ££  |__/| ££  | ££|  ££££££|  ££££££ | ££££££££|  ££££££ 
    | ££|  £££| ££  | ££| ££  | ££| ££  | ££| ££  | ££  | ££ /££|____  ££      | ££|  ££       | ££    ££| ££      | ££  | ££ |____  ££|____  ££| ££_____/ |____  ££
    | ££ |  ££|  ££££££/|  ££££££/|  £££££££| ££  | ££  |  ££££//£££££££/      |  ££££/££      |  ££££££/| ££      |  ££££££/ /£££££££//£££££££/|  £££££££ /£££££££/
    |__/  |__/ |______/  |______/  |____  ££|__/  |__/   |___/ |_______/        |____/|_/       |______/ |__/       |______/ |_______/|_______/  |_______/|_______/ 
                                   /££  | ££                                                                                                                        
                                  |  ££££££/                                                                                                                        
                                   |______/                                                                                                                         ");
    println!("How many players would you like?");
    let players: i32 = text_io::read!();
    if players == 1 {
        one_player::main()
    } else if players == 2 {
        two_players::main()
    } else {
        println!("Please only pick 1 or 2");
    };
}

#[cfg(test)]
mod tests{
    use crate::two_players::base::Winner;
    use crate::two_players::base::CellState;
    use crate::two_players::base::get_winner;
    #[test]
    fn get_winner_collumns() {
        assert_eq!(get_winner([
            [CellState::Noughts, CellState::Noughts, CellState::None],
            [CellState::Crosses, CellState::Crosses, CellState::Crosses],
            [CellState::None, CellState::None, CellState::Noughts]]
        ),Winner::Crosses)
    }
    #[test]
    fn get_winner_row() {
        assert_eq!(get_winner([
            [CellState::Noughts, CellState::Crosses, CellState::None],
            [CellState::None, CellState::Crosses, CellState::Noughts],
            [CellState::None, CellState::Crosses, CellState::Noughts]]
        ),Winner::Crosses)
    }
}
