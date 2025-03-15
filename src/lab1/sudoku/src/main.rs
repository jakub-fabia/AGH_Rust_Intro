fn main() {
    let board = [
    [5, 3, 0, 0, 7, 0, 0, 0, 0],
    [6, 0, 0, 1, 9, 5, 0, 0, 0],
    [0, 9, 8, 0, 0, 0, 0, 6, 0],
    [8, 0, 0, 0, 6, 0, 0, 0, 3],
    [4, 0, 0, 8, 0, 3, 0, 0, 1],
    [7, 0, 0, 0, 2, 0, 0, 0, 6],
    [0, 6, 0, 0, 0, 0, 2, 8, 0],
    [0, 0, 0, 4, 1, 9, 0, 0, 5],
    [0, 0, 0, 0, 8, 0, 0, 7, 9]
];
    for i in 0..9 {
        if !check_row(&board, i) || !check_col(&board, i) || !check_square(&board, i) {
            println!("The board is invalid");
            return;
        }
    }
    println!("The board is valid");
}

fn check_row(board: &[[i32; 9]; 9], row: usize) -> bool {
    let mut numbers = [0; 10];
    for i in 0..9 {
        let number: usize = board[row][i].try_into().unwrap();
        numbers[number] += 1;
        if number != 0 && numbers[number] > 1 {
            println!("Row {} is invalid", row);
            return false;
        }
    }
    true
}

fn check_col(board: &[[i32; 9]; 9], col: usize) -> bool {
    let mut numbers = [0; 10];
    for i in 0..9 {
        let number: usize = board[i][col].try_into().unwrap();
        numbers[number] += 1;
        if number != 0 && numbers[number] > 1 {
            println!("Column {} is invalid", col);
            return false;
        }
    }
    true
}

fn check_square(board: &[[i32; 9]; 9], square: usize) -> bool {
    let mut numbers = [0; 10];
    let row = square / 3;
    let col = square % 3;
    for i in 0..3 {
        for j in 0..3 {
            let number: usize = board[row * 3 + i][col * 3 + j].try_into().unwrap();
            numbers[number] += 1;
            if number != 0 && numbers[number] > 1 {
                println!("Square {} is invalid", square);
                return false;
            }
        }
    }
    true
}