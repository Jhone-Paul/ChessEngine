mod board;
mod moves;

fn main(){
    let mut board = board::new_board();
    board::print_board(&board);
    let mv_arr = ["e2e4","e7e5", "g1f3", "b8c6"];
    for mv in mv_arr{
        if moves::is_legal_move(&board, moves::parse_square(&mv[0..2]), moves::parse_square(&mv[2..4])) {
            println!("Legal Move");
            moves::make_move(&mut board, &mv);
        } else {
            println!{"Illegal Move"};
        }
        board::print_board(&board);
    }
}
