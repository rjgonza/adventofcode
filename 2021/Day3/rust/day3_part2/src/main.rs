use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut oxygen_generator_rating = "".to_string();
    let mut co2_scrubber_rating = "".to_string();
    let mut o2_binary_digits = vec![];
    let mut co2_binary_digits = vec![];
    let mut counters = vec![vec![0; 2]; 11];

    let fh = File::open("../../input.txt").unwrap();
    let reader = BufReader::new(fh);

    for (_, line) in reader.lines().enumerate() {
        let value = line.expect("Unable to parse line");
        o2_binary_digits.push(value.clone());
        co2_binary_digits.push(value);
    }

    let mut o2_number_found = false;
    let mut co2_number_found = false;
    while !(o2_number_found && co2_number_found) {
        for i in 0..11 {
            for (_index, v) in o2_binary_digits.iter().enumerate() {
                // println!("{}", v);
                match v.chars().nth(i).unwrap() {
                    '0' => counters[i][0] += 1,
                    '1' => counters[i][1] += 1,
                    _ => println!("Unmatched type"),
                }
            }
        }

        for i in 0..11 {
            let mut o2_digit = ' ';
            let mut co2_digit = ' ';
            if counters[i][0] <= counters[i][1] {
                o2_digit = '1';
                co2_digit = '0';
            } else {
                o2_digit = '0';
                co2_digit = '1';
            }

            print!("{}", co2_digit);

            // println!("O2  Digit Check #: {} -- Checking for {}", i, o2_digit);
            // println!("CO2 Digit Check #: {} -- Checking for {}", i, co2_digit);

            if o2_binary_digits.len() != 1 {
                o2_binary_digits.retain(|e| o2_digit == e.chars().nth(i).unwrap());
            } else {
                // println!("{:?} - {}", o2_binary_digits, o2_number_found);
                oxygen_generator_rating = o2_binary_digits.pop().unwrap();
                o2_number_found = true;
            }

            if co2_binary_digits.len() != 1 {
                co2_binary_digits.retain(|e| co2_digit == e.chars().nth(i).unwrap());
            } else {
                // println!("{:?} - {}", co2_binary_digits, co2_number_found);
                co2_scrubber_rating = co2_binary_digits.pop().unwrap();
                co2_number_found = true;
            }
        }
    }
    println!();

    let oxygen_generator_rating_dec = isize::from_str_radix(&oxygen_generator_rating, 2).unwrap();
    let co2_scrubber_rating_dec = isize::from_str_radix(&co2_scrubber_rating, 2).unwrap();
    let life_support_rating = oxygen_generator_rating_dec * co2_scrubber_rating_dec;
    println!(
        "Oxygen Generator Rating (binary): {} | Oxygen Generator Rating  (decimal): {}",
        oxygen_generator_rating, oxygen_generator_rating_dec
    );
    println!(
        "CO2 Scrubber Rating (binary): {} | CO2 Scrubber Rating (decimal): {}",
        co2_scrubber_rating, co2_scrubber_rating_dec
    );
    println!("Life Support Rating: {}", life_support_rating);
}
