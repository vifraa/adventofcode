
pub fn part1(input: &str) -> Result<String, &str> {
    let inputs: Vec<i32> = input.lines()
        .map(|f| f.trim().parse::<i32>().unwrap())
        .collect();

    let result = inputs.windows(2)
        .filter(|f| f[1] > f[0])
        .count();
    
    Ok(result.to_string())
}

pub fn part2(input: &str) -> Result<String, &str> {
    let inputs: Vec<i32> = input.lines()
        .map(|f| f.trim().parse().unwrap())
        .collect();

    let window_sums: Vec<i32> = inputs.windows(3)
        .map(|f| f[0] + f[1] + f[2])
        .collect();

    let result = window_sums.windows(2)
        .filter(|f| f[1] > f[0])
        .count();

    Ok(result.to_string())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        let expected = "7";

        match part1(input) {
            Ok(res) => assert_eq!(expected, res),
            Err(_) => assert!(false)
        }
    }


    #[test]
    fn test_2() {
        let input = "199
200
208
210
200
207
240
269
260
263";

        let expected = "5";

        match part2(input) {
            Ok(res) => assert_eq!(expected, res),
            Err(_) => assert!(false)
        }
    }

}


