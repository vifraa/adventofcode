
enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32)
}

pub fn part1(input: &str) -> Result<String, &str> {
    let split_input = parse_lines(input);

    let mut horizontal = 0;
    let mut vertical = 0;
    split_input.iter()
        .for_each(|f| {
            match f {
                Direction::Up(d) => vertical -= d,
                Direction::Down(d) => vertical += d,
                Direction::Forward(d) => horizontal += d
            }
        });
    
    Ok((horizontal * vertical).to_string())
}

pub fn part2(input: &str) -> Result<String, &str> {
    let split_input = parse_lines(input);

    let mut aim = 0;
    let mut horizontal = 0;
    let mut vertical = 0;

    split_input.iter()
        .for_each(|f| {
            match f {
                Direction::Up(d) => aim -= d,
                Direction::Down(d) => aim += d,
                Direction::Forward(d) => {
                    horizontal += d;
                    vertical += aim * d;
                }
            }
        });

    Ok((horizontal * vertical).to_string())
}

fn parse_lines(input: &str) -> Vec<Direction> {
    let split_input = input.lines()
        .map(|f| f.split_once(' ').unwrap())
        .filter(|(a, _)| a == &"forward" || a == &"down" || a == &"up")
        .map(|(a, b)| {
            match a {
                "forward" => Direction::Forward(b.parse().unwrap()),
                "down" => Direction::Down(b.parse().unwrap()),
                _ => Direction::Up(b.parse().unwrap()), // Since we cant have a wildcard match that returns unit value. The filter before however ensures that this logic will still be correct.
            }
        }).collect();
    split_input
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


