use crate::controls::{Input, ControlScheme};
use crate::piece::{Piece, Shapes};

pub struct Player {
    pub player_num: u8,
    pub control_scheme: ControlScheme,
    pub input: Input,
    pub active_piece: Piece,
    pub spawn_piece_flag: bool,
}

impl Player {
    pub fn new(player_num: u8, control_scheme: ControlScheme) -> Self {
        Self {
            player_num,
            control_scheme,
            input: Input::new(),
            active_piece: Piece::new(Shapes::None, player_num),
            spawn_piece_flag: true,
        }
    }
}