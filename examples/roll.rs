// Copyright 2018 Mattias Cibien
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate dicenotation;
extern crate clap;

use clap::{Arg, App};
use std::process;

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
    
    let result = dicenotation::roll_dice(notation).unwrap_or_else(|err| {
        println!("Problem parsing notation: {}", err);
        process::exit(1)
    });

    println!("Rolling: {}", notation);
    println!("\t-> {}", result);
}