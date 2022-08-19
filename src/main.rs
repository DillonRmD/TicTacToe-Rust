
fn display_game_board(game_board: [[u32; 3]; 3]){

    let mut row_count = 0;

    for(i, row) in game_board.iter().enumerate(){

        if i == 0{
            println!("  |a||b||c|")
        }

        for(j, col) in row.iter().enumerate(){
            if j == 0{
                print!("{} ", row_count);
                row_count += 1;
            }
            print!("|{}|", game_board[i][j]);
        }
        println!("");
    }

}

fn init_game_board() -> [[u32; 3]; 3]{

    let game_board = [[0; 3]; 3];
    game_board
}

fn main() {

    use std::io::{stdin};

    let game_board = init_game_board();
    display_game_board(game_board);
    let mut game_over = false;
    while game_over != true{

        let mut user_input = String::new();

        stdin().read_line(&mut user_input).expect("Issue reading input");

        if user_input.trim().eq("quit") {
            game_over = true;
        }

    }

    
}


