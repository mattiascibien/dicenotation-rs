 use super::*;

    #[test]
    fn it_parses_notation_correctly_without_modifier_i32() {
        let data = parse::<i32>("3d4").unwrap();
        assert!(data == DiceData {
            num_dice: 3i32,
            num_faces: 4i32,
            modifier: false,
            modifier_val: 0i32,
        });
    }

    #[test]
    fn it_parses_notation_correctly_with_plus_modifier_i32() {
        let data = parse::<i32>("3d4+1").unwrap();
        assert!(data == DiceData {
            num_dice: 3i32,
            num_faces: 4i32,
            modifier: true,
            modifier_val: 1i32,
        });
    }

    #[test]
    fn it_parses_notation_correctly_with_minus_modifier_i32() {
        let data = parse::<i32>("3d4-1").unwrap();
        println!("{:?}", data);
         assert!(data == DiceData {
            num_dice: 3i32,
            num_faces: 4i32,
            modifier: false,
            modifier_val: 1i32,
        });
    }

    #[test]
    fn it_does_not_parse_gibberish_i32() {
        assert!(parse::<i32>("ad1d-1").is_err());
    }