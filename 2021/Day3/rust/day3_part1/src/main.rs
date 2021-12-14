use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut gamma_rate = "".to_string();
    let mut epsilon_rate = "".to_string();
    let mut binary_digits = vec![];
    let mut counters = vec![vec![0; 2]; 12];

    let fh = File::open("../../input.txt").unwrap();
    let reader = BufReader::new(fh);

    for (_, line) in reader.lines().enumerate() {
        let value = line.expect("Unable to parse line");
        binary_digits.push(value);
    }

    for (_, v) in binary_digits.iter().enumerate() {
        // for i in 0..12 {
        for (i, c) in counters.iter_mut().enumerate() {
            // print!("{}", v.chars().nth(i).unwrap());
            match v.chars().nth(i).unwrap() {
                '0' => c[0] += 1,
                '1' => c[1] += 1,
                _ => println!("Unmatched type"),
            }
        }
    }

    for (_, c) in counters.iter_mut().enumerate() {
        if c[0] > c[1] {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }
    }

    let gamma_rate_dec = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate_dec = isize::from_str_radix(&epsilon_rate, 2).unwrap();
    let power_consumption = gamma_rate_dec * epsilon_rate_dec;
    println!(
        "Gamma Rate (binary): {} | Gamma Rate (decimal): {}",
        gamma_rate, gamma_rate_dec
    );
    println!(
        "Epsilon Rate (binary): {} | Epsilon Rate (decimal): {}",
        epsilon_rate, epsilon_rate_dec
    );
    println!("Power Consumption: {}", power_consumption);
}
