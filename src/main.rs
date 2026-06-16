//type Board = Vec<Option<Piece>>;
use std::fmt;

#[derive(Clone)]
enum Color {White, Black}

#[derive(Clone)]
enum Kind {King, Queen, Bishop, Knight, Rook, Pawn}

#[derive(Clone)]
struct Piece {
    color: Color,
    kind: Kind,
}
// handle printing for the peices
impl fmt::Display for Piece {
    fn fmt(&self, f :&mut fmt::Formatter)-> fmt::Result {
        let symbol = match (&self.color, &self.kind) {
            (Color::Black, Kind::King)   => "♔",
            (Color::Black, Kind::Queen)  => "♕",
            (Color::Black, Kind::Rook)   => "♖",
            (Color::Black, Kind::Bishop) => "♗",
            (Color::Black, Kind::Knight) => "♘",
            (Color::Black, Kind::Pawn)   => "♙",
            (Color::White, Kind::King)   => "♚",
            (Color::White, Kind::Queen)  => "♛",
            (Color::White, Kind::Rook)   => "♜",
            (Color::White, Kind::Bishop) => "♝",
            (Color::White, Kind::Knight) => "♞",
            (Color::White, Kind::Pawn)   => "♟",
        };
        write!(f, "{}", symbol)
    }
}

type Board = Vec<Option<Piece>>;

fn new_board()->Board{
    let mut board:Board  = vec![None; 64];
    let back_rank= [Kind::Rook, Kind::Knight, Kind::Bishop, Kind::Queen, Kind::King, Kind::Bishop, Kind:: Knight, Kind::Rook];
    
    for (i, kind) in back_rank.into_iter().enumerate() {
        board[i]      = Some(Piece { color: Color::Black, kind: kind.clone() });
        board[8 + i]  = Some(Piece { color: Color::Black, kind: Kind::Pawn });
        board[48 + i] = Some(Piece { color: Color::White, kind: Kind::Pawn });
        board[56 + i] = Some(Piece { color: Color::White, kind });
    }

    board
}

fn print_board(board: &Board){
    println!(" |{:^5}|{:^5}|{:^5}|{:^5}|{:^5}|{:^5}|{:^5}|{:^5}|", "a", "b", "c", "d", "e", "f", "g", "h");
    println!("{}","-".repeat(50));

    for i in 0..8{
        print!("{}|",i);
        for j in 0..8 {
            let cell = match &board[i * 8 + j] {
                Some(piece) => format!("{}", piece),
                None        => " ".to_string(),
            };
            print!("{:^5}|", cell);
        }
        println!{};
        println!("{}","-".repeat(50));
    }

}


fn main() {
    let board = new_board();
    print_board(&board);
}
