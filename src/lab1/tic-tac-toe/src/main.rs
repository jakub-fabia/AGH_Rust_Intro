use std::io::{self, Write};

fn main() {
    let mut board = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
    let mut player = 0;
    let mut move_num = 1;

    print_board(board);

    while !is_over(board, move_num) {
        let cmd = get_player_move(player);
        let (row, col) = ((cmd - 1) / 3, (cmd - 1) % 3);

        if board[row][col] == 'X' || board[row][col] == 'O' {
            println!("Invalid move. Please choose an empty cell.");
            continue;
        }

        board[row][col] = if player == 0 { 'X' } else { 'O' };
        player = 1 - player;
        print_board(board);
        move_num += 1;
    }
    println!("Koniec gry!");
}

fn get_player_move(player: usize) -> usize {
    let player_message = ["Gracz 1 (X)", "Gracz 2 (O)"];
    loop {
        print!("{}, Twój ruch: ", player_message[player]);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(cmd) = input.trim().parse::<usize>() {
            if cmd >= 1 && cmd <= 9 {
                return cmd;
            }
        }
        println!("Invalid input. Please choose a number between 1 and 9.");
    }
}

fn print_board(board: [[char; 3]; 3]) {
    println!("Plansza:");
    for row in 0..3 {
        println!("{} | {} | {}", board[row][0], board[row][1], board[row][2]);
        if row < 2 {
            println!("---------");
        }
    }
}


fn is_over(board: [[char; 3]; 3], move_num: i16) -> bool {
    for i in 0..3 {
        if board[i][0] == board[i][1] && board[i][1] == board[i][2] {
            return announce_winner(board[i][0]);
        }
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            return announce_winner(board[0][i]);
        }
    }
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return announce_winner(board[0][0]);
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return announce_winner(board[0][2]);
    }
    if move_num == 10 {
        println!("Remis!");
        return true;
    }
    false
}

fn announce_winner(player: char) -> bool {
    if player == 'X' {
        println!("Gracz 1 wygrywa!");
    } else {
        println!("Gracz 2 wygrywa!");
    }
    true
}