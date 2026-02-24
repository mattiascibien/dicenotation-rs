// Copyright 2018 Mattias Cibien
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use clap::{arg, Command};

fn main() {
    let matches = Command::new("MyApp")
        .version("1.0")
        .author("Mattias Cibien <mattias@mattiascibien.net>")
        .about("Rolls dice using Standard Dice Notation")
        .arg(
            arg!(<notation> "The dice notation to roll")
                .required(true)
                .index(1),
        )
        .get_matches();

    let notation = matches.get_one::<String>("notation").unwrap();

    let result = dicenotation::roll_dice::<i32>(notation).unwrap();

    println!("Rolling: {}", notation);
    println!("\t-> {}", result);
}
