extern crate dicenotation;
extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Dice Roller")
                .version("1.0")
                .author("Mattias Cibien <mattias@mattiascibien.net>")
                .about("Rolls dice using Standard Dice Notation")
                .arg(Arg::with_name("dicestring")
                    .required(true)
                    .index(1)
                    .takes_value(true))
                .get_matches();

     let dicestring = matches.value_of("dicestring").unwrap();

     println!("Rolling: {}", dicestring);
     // TODO: roll the dice using the library
}