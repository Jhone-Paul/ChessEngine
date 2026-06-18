mod board;
mod moves;
mod io;

fn main(){
    let mut board = board::new_board();
    board::print_board(&board);
    loop {
        let mv = io::get_move_command_line(&board::Color::White);
        if moves::is_legal_move(&board, moves::parse_square(&mv.mv[0..2]), moves::parse_square(&mv.mv[2..4])) {
            println!("Legal Move");
            moves::make_move(&mut board, &mv.mv);
        } else {
            println!{"Illegal Move"};
        }
        board::print_board(&board);
    } 
}
