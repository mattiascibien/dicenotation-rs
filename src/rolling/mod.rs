// Copyright 2018 Mattias Cibien
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::rand;
use super::rand::{Rng};
use super::DiceData;

pub fn roll(dice_data: DiceData) -> u32 {
    let mut rng = rand::thread_rng();

    let mut result = 0;

    for _i in 0..dice_data.num_dice {
        let roll : u32 = rng.gen_range(0, dice_data.num_faces) + 1;
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
mod tests {
    use super::*;

    #[test]
    fn it_rolls_two_dices_of_three_faces_with_modifier_plus_two_correctly() {
        let data = DiceData {
            num_dice: 2,
            num_faces: 3,
            modifier: true,
            modifier_val: 2
        };

        let result = roll(data);

        assert!(result >= 4);
        assert!(result <= 8);
    }

    #[test]
    fn it_rolls_two_dices_of_three_faces_with_modifier_minus_two_correctly() {
        let data = DiceData {
            num_dice: 2,
            num_faces: 3,
            modifier: false,
            modifier_val: 2
        };

        let result = roll(data);

        assert!(result >= 0);
        assert!(result <= 4);
    }
}