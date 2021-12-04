use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut oxygen_generator_rating = "".to_string();
    let mut co2_scrubber_rating = "".to_string();
    let mut o2_binary_digits = vec![];
    let mut co2_binary_digits = vec![];

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
        for i in 0..12 {
            let mut counter_0 = 0;
            let mut counter_1 = 0;
            for (_, v) in o2_binary_digits.iter().enumerate() {
                match v.chars().nth(i).unwrap() {
                    '0' => counter_0 += 1,
                    '1' => counter_1 += 1,
                    _ => println!("Unmatched type"),
                }
            }

            // println!("i: 1:{} | 0:{}", counter_1, counter_0);

            if counter_1 >= counter_0 {
                o2_binary_digits.retain(|e| e.chars().nth(i).unwrap() == '1');
            } else {
                o2_binary_digits.retain(|e| e.chars().nth(i).unwrap() == '0');
            }

            // println!("{:?}", o2_binary_digits);

            if o2_binary_digits.len() == 1 {
                oxygen_generator_rating = o2_binary_digits.pop().unwrap();
                o2_number_found = true;
                break;
            }
        }

        for i in 0..12 {
            let mut counter_0 = 0;
            let mut counter_1 = 0;
            for (_, v) in co2_binary_digits.iter().enumerate() {
                match v.chars().nth(i).unwrap() {
                    '0' => counter_0 += 1,
                    '1' => counter_1 += 1,
                    _ => println!("Unmatched type"),
                }
            }

            // println!("i: 1:{} | 0:{}", counter_1, counter_0);

            if counter_1 >= counter_0 {
                co2_binary_digits.retain(|e| e.chars().nth(i).unwrap() == '0');
            } else {
                co2_binary_digits.retain(|e| e.chars().nth(i).unwrap() == '1');
            }

            if co2_binary_digits.len() == 1 {
                co2_scrubber_rating = co2_binary_digits.pop().unwrap();
                co2_number_found = true;
            }
        }
    }

    let oxygen_generator_rating_dec = isize::from_str_radix(&oxygen_generator_rating, 2).unwrap();
    let co2_scrubber_rating_dec = isize::from_str_radix(&co2_scrubber_rating, 2).unwrap();
    let life_support_rating = oxygen_generator_rating_dec * co2_scrubber_rating_dec;
    println!(
        "Oxygen Generator Rating (binary): {} | Oxygen Generator Rating (decimal): {}",
        oxygen_generator_rating, oxygen_generator_rating_dec
    );
    println!(
        "CO2 Scrubber Rating (binary):     {} | CO2 Scrubber Rating (decimal):     {}",
        co2_scrubber_rating, co2_scrubber_rating_dec
    );
    println!();
    println!("Life Support Rating: {}", life_support_rating);
}
