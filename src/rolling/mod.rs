use super::rand;
use super::rand::{Rng};
use super::DiceData;

pub fn roll(dice_data: DiceData) -> u32 {
    let mut rng = rand::thread_rng();

    let mut result = 0;

    for _i in 0..dice_data.num_dice {
        let roll : u32 = rng.gen_range(1, dice_data.num_faces);
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
    fn it_rolls_correcly() {
        let data = DiceData {
            num_dice: 2,
            num_faces: 3,
            modifier: true,
            modifier_val: 2
        };

        let result = roll(data);

        assert!(result >= 5);
        assert!(result <= 8);
    }
}