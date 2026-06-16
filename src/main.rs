mod board;
mod moves;

fn main(){
    let mut board = board::new_board();
    board::print_board(&board);
    let mv = "e2e4";
    moves::make_move(&mut board, &mv);
    board::print_board(&board);
}
