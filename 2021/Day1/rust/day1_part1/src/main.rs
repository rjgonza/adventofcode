#![allow(unused_variables, unused_assignments)]
use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let matches = App::new("2021 AoC Day 1 Part 1 Solution")
        .version("0.1")
        .author("Ramon Gonzalez")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .multiple(false)
                .help("File location for the input"),
        )
        .get_matches();

    let file = matches.value_of("file").unwrap();

    let fh = File::open(file).unwrap();
    let reader = BufReader::new(fh);
    let mut previous = 0;
    let mut increase = 0;

    for (count, line) in reader.lines().enumerate() {
        let value = line.expect("Unable to parse line");
        let res;
        if count == 0 {
            previous = value.parse::<i32>().unwrap();
            res = "N/A";
        } else if previous < value.parse::<i32>().unwrap() {
            increase += 1;
            res = "increase";
        } else {
            res = "decrease";
        }

        previous = value.parse::<i32>().unwrap();

        println!("{} ({})", value, res)
    }

    println!("increase: {}", increase);
}
