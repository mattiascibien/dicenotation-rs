extern crate rand;

mod rolling;

pub struct DiceData {
    num_dice: u32,
    num_faces: u32,
    modifier: bool,
    modifier_val: u32
}

