// Copyright 2018 Mattias Cibien
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Library aimed at parsing and rolling dices 
//! for Role-Playing games using [Standard Dice Notation](https://en.wikipedia.org/wiki/Dice_notation#Standard_notation) 
//! that bears inspiration from the  [dicenotation](https://github.com/mattiascibien/dicenotation) library. 

#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate rand;
extern crate num_traits;
extern crate num_iter;

use std::str::FromStr;
use num_traits::int::PrimInt;
use std::fmt::Debug;
use rand::distributions::range::SampleRange;

mod rolling;
mod parsing;

/// Struct represeting a die roll data
#[derive(Debug)]
pub struct DiceData<T> where T: PrimInt {
    /// Number of die to roll
    num_dice: T,
    /// Number of faces of each dice
    num_faces: T,
    /// Modifies (true for plus, false for minus)
    modifier: bool,
    /// Modifier value (alters the result of the die roll)
    modifier_val: T
}

impl<T> PartialEq for DiceData<T> where T: PrimInt {
    fn eq(&self, other: &DiceData<T> ) -> bool {
        self.num_dice == other.num_dice
        && self.num_faces == other.num_faces
        && self.modifier == other.modifier
        && self.modifier_val == other.modifier_val
    }
}

/// Execute a dice roll based on the given notation
/// 
/// # Examples
/// 
/// Gets the result of rolling 3 die of 5 faces
/// 
/// ```
/// use dicenotation::roll_dice;
/// 
/// let result = roll_dice::<i32>("3d5");
/// ```
/// 
/// Executes two rolls by summing their values
/// 
/// ```
/// use dicenotation::roll_dice;
/// 
/// let result = roll_dice::<i32>("3d5").unwrap() + roll_dice::<i32>("2d3").unwrap();
/// ``` 
pub fn roll_dice<T>(notation : &str) -> Result<T, &str> where T : PrimInt + FromStr + SampleRange, <T as FromStr>::Err : Debug {
    let dice_data = parsing::parse(notation);

    let dice_data = match dice_data {
        Ok(d) => d,
        Err(e) => return Err(e)
    };

    let result = rolling::roll(dice_data);

    Ok(result)
}