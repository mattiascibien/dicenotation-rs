// Copyright 2018 Mattias Cibien
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::rand;
use super::rand::{Rng};
use super::rand::distributions::range::SampleRange;
use super::DiceData;

use super::num_traits::Num;
use super::num_traits::ToPrimitive;
use super::num_traits::zero;
// TODO: not sure if this is correct
use super::num_traits::one;
use super::num_iter::range;

pub fn roll<T>(dice_data: DiceData<T>) -> T where T : Num + PartialOrd + SampleRange + Copy + Clone + ToPrimitive {
    let mut rng = rand::thread_rng();

    let mut result : T = zero();

    for _i in range(zero(), dice_data.num_dice) {
        let roll : T = rng.gen_range(zero(), dice_data.num_faces) + one();
        result = result + roll;
    }

    if dice_data.modifier {
        result = result + dice_data.modifier_val;
    } else {
        result = result - dice_data.modifier_val;
    }

    result
}

#[cfg(test)]
mod test;