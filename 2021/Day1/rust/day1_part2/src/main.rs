#![allow(unused_variables, unused_assignments)]
extern crate getopts;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let name = Path::new(&program).file_name().unwrap().to_str().unwrap();

    let mut opts = Options::new();
    opts.optopt("f", "file", "location of the input file", "FILE");
    opts.optflag("h", "help", "display the help output");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f);
        }
    };
    if matches.opt_present("h") {
        println!(
            "{}

    {} -h - This help text
    {} -f <filename> - the file to use as input
",
            name, name, name
        );
        return;
    }
    let file = matches.opt_str("f");

    let fh = File::open(file.unwrap()).unwrap();
    let reader = BufReader::new(fh);

    let mut increase = 0;
    let mut previous1 = 0;
    let mut previous2 = 0;
    let mut previous_sum = 0;

    for (count, line) in reader.lines().enumerate() {
        let value = line.expect("Unable to parse line");
        let mut res = "";

        let sum = previous1 + previous2 + value.parse::<i32>().unwrap();

        if count < 3 {
            previous1 = value.parse::<i32>().unwrap();
            res = "N/A"
        } else if previous_sum < sum {
            res = "increase";
            increase += 1;
        } else {
            res = "decrease";
        }

        println!("{} ({})", sum, res);

        previous1 = previous2;
        previous2 = value.parse::<i32>().unwrap();
        previous_sum = sum;
    }
    println!("Increases: {}", increase);
}
