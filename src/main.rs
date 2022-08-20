
fn mark_spot_on_game_board(mut game_board: [[u32; 3]; 3], i: &str, j: usize) -> [[u32; 3]; 3]{
    
    let mut col_input = 0;

    if i.eq("a") {
        col_input = 0;
    }
    else if i.eq("b"){
        col_input = 1;
    }
    else if i.eq("c"){
        col_input = 2;
    }

    game_board[j][col_input] = 1;

    game_board
}

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

    let mut game_board = init_game_board();
    

    let mut game_over = false;
    while game_over != true{

        display_game_board(game_board);

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).expect("Issue reading input");
        
        if user_input.trim().eq("quit") {
            game_over = true;
        }

        let parsed_input: Vec<&str> = user_input.trim().split('-').collect();

        let col_input = parsed_input[0];
        let row_input = parsed_input[1].parse::<usize>().unwrap();
        game_board = mark_spot_on_game_board(game_board, col_input, row_input);
    }

    
}


