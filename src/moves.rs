use crate::board::Board;

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
