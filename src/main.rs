use std::io;

fn main() {

    println!("Hello, World!");

    let name = "Kristian";

    println!("Hello, {}", name);
     let mut board = [' '; 9];

    board[0] = 'X';
    board[4] = 'X';
    board[8] = 'X';

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
    );

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error reading input");

    println!("{input}")



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