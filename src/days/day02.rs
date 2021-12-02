
pub fn part1(input: &str) -> Result<String, &str> {
    let split_input: Vec<Vec<&str>> = input.lines()
        .map(|f| f.split_whitespace().collect())
        .collect();

    let horizontal_change: i32 = split_input.iter()
        .filter(|f| f[0] == "forward")
        .map(|f| f[1].parse::<i32>().unwrap())
        .sum();

    let vertical_change: i32 = split_input.iter()
        .filter(|f| f[0] != "forward")
        .map(|f| {
            match f[0] {
                "down" => f[1].parse::<i32>().unwrap(),
                "up" => -f[1].parse::<i32>().unwrap(),
                _ => 0
            }
        }).sum();

    Ok((horizontal_change * vertical_change).to_string())
}

pub fn part2(input: &str) -> Result<String, &str> {
    let split_input: Vec<Vec<&str>> = input.lines()
        .map(|f| f.split_whitespace().collect())
        .collect();


    let mut aim = 0;
    let mut horizontal = 0;
    let mut vertical = 0;
    for s in split_input {
        match s[0] {
            "down" => {
                aim += s[1].parse::<i32>().unwrap();
            },
            "up" => {
                aim -= s[1].parse::<i32>().unwrap();
            },
            "forward" => {
                horizontal += s[1].parse::<i32>().unwrap();
                vertical += aim*s[1].parse::<i32>().unwrap();
            },
            _ => ()
        };
    }

    Ok((horizontal * vertical).to_string())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let expected = "150";

        match part1(input) {
            Ok(res) => assert_eq!(expected, res),
            Err(_) => assert!(false)
        }
    }


    #[test]
    fn test_2() {
let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let expected = "900";

        match part2(input) {
            Ok(res) => assert_eq!(expected, res),
            Err(_) => assert!(false)
        }
    }

}


