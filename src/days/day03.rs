use std::usize;


pub fn part1(input: &str) -> Result<String, &str> {
    let inputs: Vec<&str> = input.lines().collect();

    let mut counter = vec![0; inputs[0].len()];
    inputs.iter()
        .for_each(|f| {
            for (i, v) in f.chars().enumerate() {
                match v {
                    '1' => counter[i] += 1,
                    '0' => counter[i] -= 1,
                    _ => ()
                }
            }
        });

    let gamma_binary = counter.iter()
        .map(|f| {
            if f < &0 {
                "0"
            } else {
                "1"
            }
        }).collect::<Vec<&str>>().join("");
    let epsilon_binary = gamma_binary.chars().map(|f| {
        if f == '0' {
            "1"
        } else {
            "0"
        }
    }).collect::<Vec<&str>>().join("");


    let gamma = usize::from_str_radix(&gamma_binary, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_binary, 2).unwrap();

    Ok((gamma*epsilon).to_string())
}

pub fn part2(input: &str) -> Result<String, &str> {

    Err("Not implemented")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let expected = "198";

        match part1(input) {
            Ok(res) => assert_eq!(expected, res),
            Err(_) => assert!(false)
        }
    }


    #[test]
    fn test_2() {
let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let expected = "";

        match part2(input) {
            Ok(res) => assert_eq!(expected, res),
            Err(_) => assert!(false)
        }
    }

}


