#[macro_use] extern crate clap;

mod lib;

use std::fs::File;
use std::io::prelude::*;

const ADVENT_VERSION: &'static str = "0.1.0";

fn main() {
    let matches = clap_app!(myapp =>
                            (version: ADVENT_VERSION)
                            (author: "salpalvv <gth701m@gmail.com>")
                            (about: "advent: Advent of Code 2016 solver")
                            (@arg day: -d --day +takes_value "Sets day to solve")
                            (@arg input: -i --input +takes_value "Sets input to take in")
                            ).get_matches();

    let day: String = matches.value_of("day").unwrap_or("1").parse().unwrap();
    let day_val: i32 = day.parse().expect("Wanted a number");
    let input_file: String = matches.value_of("input").unwrap_or("inputs/day1").parse().unwrap();

    let mut file = File::open(&input_file)
        .expect("could not open file");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("could not read file");

    let answer = lib::solve(&day_val, &input);
    println!("The answer for day {} is {}", &day, answer);

}
