// Copyright 2018 Mattias Cibien
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate rand;

mod rolling;
mod parsing;

#[derive(Debug)]
pub struct DiceData {
    num_dice: u32,
    num_faces: u32,
    modifier: bool,
    modifier_val: u32
}

impl PartialEq for DiceData {
    fn eq(&self, other: &DiceData) -> bool {
        self.num_dice == other.num_dice
        && self.num_faces == other.num_faces
        && self.modifier == other.modifier
        && self.modifier_val == other.modifier_val
    }
}

pub fn roll_dice(notation : &str) -> u32 {
    let dice_data = parsing::parse(notation);
    let result = rolling::roll(dice_data);

    result
}