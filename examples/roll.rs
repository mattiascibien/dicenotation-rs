// Copyright 2018 Mattias Cibien
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate dicenotation;
extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Dice Roller")
                .version("1.0")
                .author("Mattias Cibien <mattias@mattiascibien.net>")
                .about("Rolls dice using Standard Dice Notation")
                .arg(Arg::with_name("notation")
                    .required(true)
                    .index(1)
                    .takes_value(true))
                .get_matches();

     let notation = matches.value_of("notation").unwrap();

     println!("Rolling: {}", notation);
     // TODO: roll the dice using the library
     println!("\t-> {}", dicenotation::roll_dice(notation) )
}