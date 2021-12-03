extern crate getopts;
use getopts::Options;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{env, result};

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

    let mut x = 0;
    let mut y = 0;

    for line in reader.lines().enumerate() {
        let value = line.1.expect("Unable to parse line");

        let result = value.to_string();
        let data: Vec<&str> = result.split(' ').collect();

        match data[0] {
            "forward" => x += data[1].parse::<i32>().unwrap(),
            "down" => y += data[1].parse::<i32>().unwrap(),
            "up" => y -= data[1].parse::<i32>().unwrap(),
            _ => println!("Errrr"),
        }

        println!("x: {}, y: {}", x, y);
    }

    println!("Position: {}", x * y);
}
