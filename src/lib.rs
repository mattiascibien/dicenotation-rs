// Copyright 2018 Mattias Cibien
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate rand;

mod rolling;

pub struct DiceData {
    num_dice: u32,
    num_faces: u32,
    modifier: bool,
    modifier_val: u32
}

