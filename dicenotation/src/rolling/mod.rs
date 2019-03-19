// Copyright 2018 Mattias Cibien
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use rand;
use rand::distributions::range::SampleRange;
use rand::Rng;
use super::DiceData;

use num_traits::int::PrimInt;
use num_traits::zero;
// TODO: not sure if this is correct
use num_iter::range;
use num_traits::one;

pub fn roll<T>(dice_data: DiceData<T>) -> T
where
    T: PrimInt + SampleRange,
{
    let mut rng = rand::thread_rng();

    roll_with_fn(dice_data, |a, b| { rng.gen_range(a,b) })
}

pub fn roll_with_fn<T,F>(dice_data: DiceData<T>, mut function: F) -> T
where
    T: PrimInt + SampleRange,
    F: FnMut(T,T) -> T,
{
    let mut result: T = zero();

    for _i in range(zero(), dice_data.num_dice) {
        let roll: T = function(zero(), dice_data.num_faces) + one();
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
