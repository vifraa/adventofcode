use std::fs;
use std::process;

use clap::{Arg, App};

fn main() {
    
    let matches = App::new("Advent of Code")
        .author("Viktor Franzén. <viktor@frnzn.com>")
        .about("Solutions to advent of code problems.")
        .arg(Arg::new("day")
             .short('d')
             .long("day")
             .about("The day to run")
             .required(true)
             .takes_value(true)
             )
        .arg(Arg::new("part")
             .short('p')
             .long("part")
             .about("The part of the day to run")
             .required(true)
             .takes_value(true))
        .arg(Arg::new("input")
             .short('i')
             .long("input")
             .about("The path to the input")
             .required(true)
             .takes_value(true))
        .get_matches();


    let input_path = matches.value_of("input").unwrap();
    let input = &fs::read_to_string(input_path).expect(&format!("Could not find input at path: {}", input_path));

    // Should probably handle these more gracefully than just unwrapping
    // Also should probably check here if they are of the correct format.
    let day = matches.value_of("day").unwrap();
    let part = matches.value_of("part").unwrap();

    
    let result = match day {
        "1" => {
            match part {
                "1" => adventofcode::days::day01::part1(input),
                "2" => adventofcode::days::day01::part2(input),
                _ => Err("Part not found")
            }
        },
        "2" => {
            match part {
                "1" => adventofcode::days::day02::part1(input),
                "2" => adventofcode::days::day02::part2(input),
                _ => Err("Part not found")
            }
        }
        _ => Err("Day not found")
    };


    match result {
        Ok(res) => println!("Day {}, Part {}: {}", day, part, res),
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }
}

