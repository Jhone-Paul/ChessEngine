use std::io;
use crate::board::Color;

#[derive(Clone)]
pub struct Move {
    pub color: Color,
    pub mv: String,
}

pub fn get_move_command_line(pcolor: &Color) -> Move {
    let mut input = String::with_capacity(4);

    let _ = io::stdin().read_line(&mut input);
    let mv = Move {
        color: pcolor.clone(),
        mv: input,
    };

    mv
}

