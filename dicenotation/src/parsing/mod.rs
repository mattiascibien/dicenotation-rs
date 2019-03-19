use super::regex::Regex;
use super::DiceData;

use super::num_traits::int::PrimInt;
use std::fmt::Debug;
use std::str::FromStr;

pub fn parse<T>(notation: &str) -> Result<DiceData<T>, &str>
where
    T: PrimInt + FromStr,
    <T as FromStr>::Err: Debug,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9]+)d([0-9]+)([+-]){0,1}([0-9]+){0,1}").unwrap();
    }

    let captures = match RE.captures(notation) {
        Some(c) => c,
        None => return Err("cannot parse"),
    };

    let mut data = DiceData {
        num_dice: captures[1].parse::<T>().unwrap(),
        num_faces: captures[2].parse::<T>().unwrap(),
        modifier: false,
        modifier_val: T::zero(),
    };

    if let Some(modifier) = captures.get(3) {
        data.modifier = modifier.as_str() == "+";
    }

    if let Some(modifier_val) = captures.get(4) {
        data.modifier_val = modifier_val.as_str().parse::<T>().unwrap();
    }

    Ok(data)
}

#[cfg(test)]
mod test;
