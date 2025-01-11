use std::io;

fn main() {

    let mut board = [' '; 9];

    board[0] = 'X';
    board[4] = 'X';
    board[8] = 'X';

    print_board(board);
    loop {
        let input = get_input();
        if input.trim() == "quit" {break;}
        println!("{input}")
    }
    
}

fn print_board(board: [char;9]){
    println!(
        "

        +---+---+---+
        | {} | {} | {} |
        +---+---+---+
        | {} | {} | {} |
        +---+---+---+
        | {} | {} | {} |
        +---+---+---+
        ",
        board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]
    )
}

fn get_input() -> String{
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error whilst reading the input");
    

    input
}