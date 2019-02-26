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

    // assert!(result >= 0); TODO: check when we extend to unsigned
    assert!(result <= 4);
}