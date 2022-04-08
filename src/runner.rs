use crate::hexapawn::{game::*, interface::*};

pub fn run() {
    let h: Hexapawn = Hexapawn::new();
    h.print_board();
}