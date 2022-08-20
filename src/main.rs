use rand::Rng;
use std::io::{stdin};

struct GameState{
    game_over: bool,
    game_board: [[u32; 3]; 3],
    user_input: Vec<usize>
}

fn mark_spot_for_ai(mut game_state: GameState) -> GameState{
    
    let mut rng = rand::thread_rng();


    let mut mark_is_valid = false;

    while mark_is_valid != true{

        let ai_col_input: usize = rng.gen_range(0..3);
        let ai_row_input: usize = rng.gen_range(0..3);

        let j = ai_col_input;
        let i = ai_row_input;
        
        if game_state.game_board[i][j] == 0 {
            game_state.game_board[i][j] = 2;
            mark_is_valid = true;
        }
    }
    
    game_state

}

fn mark_spot_for_user(mut game_state: GameState) -> GameState{
    
    let mut mark_is_valid = false;

    while mark_is_valid != true{

        game_state = get_user_input(game_state);

        let j = game_state.user_input[0];
        let i = game_state.user_input[1];

        if game_state.game_board[i][j] != 0 {
            println!("Space already occupied!");
        }
        else{
            game_state.game_board[i][j] = 1;
            mark_is_valid = true;
        }
    }
    
    game_state
}

fn get_user_input(mut game_state: GameState) -> GameState{

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).expect("Issue reading input");
    
    if user_input.trim().eq("quit") {
        game_state.game_over = true;
    }
    
    let parsed_input: Vec<&str> = user_input.trim().split('-').collect();
    
    let col_input = convert_character_to_integer(parsed_input[0]);
    let row_input = parsed_input[1].parse::<usize>().unwrap();
    game_state.user_input = vec![col_input, row_input];

    game_state
}

fn convert_character_to_integer(char: &str) -> usize{
    
    let mut result: usize = 0;

    if char.eq("a") {
        result = 0;
    }
    else if char.eq("b"){
        result = 1;
    }
    else if char.eq("c"){
        result = 2;
    }

    result
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

    let mut game_state = GameState{
        game_over: false,
        game_board: init_game_board(),
        user_input: Vec::new()
    };    

    while game_state.game_over != true{

        display_game_board(game_state.game_board);
        game_state = mark_spot_for_user(game_state);
        game_state = mark_spot_for_ai(game_state);
    }

    
}


