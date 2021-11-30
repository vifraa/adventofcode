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
        .get_matches();


    // Should probably handle these more gracefully than just unwrapping
    // Also should probably check here if they are of the correct format.
    let day = matches.value_of("day").unwrap();
    let part = matches.value_of("part").unwrap();




}

