use std::fmt::Display;


fn DisplayGameBoard(game_board: [[u32; 3]; 3]){

    for(i, row) in game_board.iter().enumerate(){
        for(j, col) in row.iter().enumerate(){
            print!("|{}|", game_board[i][j]);
        }
        println!("");
    }

}

fn main() {

    let mut game_board = InitGameBoard();

    DisplayGameBoard(game_board);
}

fn InitGameBoard() -> [[u32; 3]; 3]{

    let game_board = [[0; 3]; 3];

    game_board
}
