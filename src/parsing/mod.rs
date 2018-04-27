use super::regex::Regex;
use super::DiceData;

pub fn parse(notation : &str) -> Result<DiceData, &str> {

    lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9]+)d([0-9]+)([+-]){0,1}([0-9]+){0,1}").unwrap();
    }

    let captures = match RE.captures(notation) {
        Some(c) => c,
        None => return Err("cannot parse")
    };

    let mut data = DiceData {
        num_dice: captures[1].parse::<u32>().unwrap(),
        num_faces: captures[2].parse::<u32>().unwrap(),
        modifier: false,
        modifier_val: 0,
    };

    if let Some(modifier) = captures.get(3) {
        data.modifier = modifier.as_str() == "+";
    } 

    if let Some(modifier_val) = captures.get(4) {
        data.modifier_val = modifier_val.as_str().parse::<u32>().unwrap();
    } 

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_notation_correctly_without_modifier() {
        let data = parse("3d4").unwrap();
        assert!(data == DiceData {
            num_dice: 3,
            num_faces: 4,
            modifier: false,
            modifier_val: 0,
        });
    }

    #[test]
    fn it_parses_notation_correctly_with_plus_modifier() {
        let data = parse("3d4+1").unwrap();
        assert!(data == DiceData {
            num_dice: 3,
            num_faces: 4,
            modifier: true,
            modifier_val: 1,
        });
    }

    #[test]
    fn it_parses_notation_correctly_with_minus_modifier() {
        let data = parse("3d4-1").unwrap();
        println!("{:?}", data);
         assert!(data == DiceData {
            num_dice: 3,
            num_faces: 4,
            modifier: false,
            modifier_val: 1,
        });
    }

    #[test]
    fn it_does_not_parse_gibberish() {
        assert!(parse("ad1d-1").is_err());
    }
}