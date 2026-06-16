use crate::board::Board;
use crate::board::Color;
use crate::board::Piece;


pub fn make_move(board: &mut Board, str_move: &str){
    let from = parse_square(&str_move[0..2]);
    let to   = parse_square(&str_move[2..4]);

    let piece = board[from].take();

    board[to] = piece;
}

fn parse_square(s:&str) -> usize{
    let mut chars = s.chars();
    let col = chars.next().unwrap();
    let row = chars.next().unwrap();

    let col = (col as usize) - ('a' as usize);
    let row = (row as usize) - ('1' as usize);

    row * 8 + col
    
}

// Funtions for each piece. 

fn is_path_clear(board: &Board, from: usize, to: usize) -> bool {
    let from_row = (from / 8) as i32;
    let from_col = (from % 8) as i32;
    let to_row   = (to / 8) as i32;
    let to_col   = (to % 8) as i32;

    let row_step = (to_row - from_row).signum(); // -1, 0, or 1
    let col_step = (to_col - from_col).signum();

    let mut row = from_row + row_step;
    let mut col = from_col + col_step;

    while (row, col) != (to_row, to_col) {
        if board[(row * 8 + col) as usize].is_some() { return false; }
        row += row_step;
        col += col_step;
    }
    true
}

fn is_legal_knight(from:usize, to:usize) -> bool {
     let from_row = (from/8) as i32;
     let from_col = (from%8) as i32;
     let to_row = (to/8) as i32;
     let to_col = (to%8) as i32;

     let row_diff = (from_row - to_row).abs();
     let col_diff = (from_col - to_col).abs();
     (row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2)
}

fn is_legal_rook(board: &Board, from:usize, to:usize) -> bool{
    let from_row = (from/8) as i32;
    let from_col = (from%8) as i32;
    let to_row = (to/8) as i32;
    let to_col = (to%8) as i32;

    if from_row != to_row && from_col != to_col { return false; }

    is_path_clear(board, from, to) 
}

fn is_legal_bishop(board: &Board, from:usize, to:usize) -> bool{
    let from_row = (from / 8) as i32;
    let from_col = (from % 8) as i32;
    let to_row   = (to / 8) as i32;
    let to_col   = (to % 8) as i32;

    if (from_row - to_row).abs() != (from_col - to_col).abs() { return false; }

    is_path_clear(board, from, to)
}

fn is_legal_pawn(board: &Board, from:usize, to:usize) -> bool {
    let from_row = (from/8) as i32;
    let from_col = (from%8) as i32;
    let to_row = (to/8) as i32;
    let to_col = (to%8) as i32;
    
    let piece = match &board[from] {
        Some(p) => p,
        None => return false,
    };

    let direction = match piece.color{
        Color::White => 1,
        Color::Black => -1,
    };
    
    let start_row = match piece.color{
        Color::White => 2,
        Color::Black => 6,
    };
    
    let row_diff = to_row - from_row;
    let col_diff = (to_col - from_col).abs();

    if col_diff == 0 && row_diff == direction {
        return board[to].is_none();
    }

    if col_diff == 0 && row_diff == 2 * direction && from_row == start_row {
        let middle = ((from_row + direction) * 8 + from_col) as usize;
        return board[to].is_none() && board[middle].is_none();
    }

    if col_diff == 1 && row_diff == direction {
        return board[to].is_some();
    }

    false

}

fn is_leagal_king(from:usize, to:usize)-> bool{
    let from_row = (from/8) as i32;
    let from_col = (from%8) as i32;
    let to_row = (to/8) as i32;
    let to_col = (to%8) as i32;

    let row_diff = (from_row - to_row).abs();
    let col_diff = (from_col - to_col).abs();

    row_diff <=1 && col_diff <=1
}
