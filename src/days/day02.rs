
pub fn part1(input: &str) -> Result<String, &str> {
    Ok("Not implemented".to_string())
}

pub fn part2(input: &str) -> Result<String, &str> {
    Ok("Not implemented".to_string())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "";
        let expected = "";

        match part1(input) {
            Ok(res) => assert_eq!(expected, res),
            Err(_) => assert!(false)
        }
    }


    #[test]
    fn test_2() {
        let input = "";        
        let expected = "";

        match part2(input) {
            Ok(res) => assert_eq!(expected, res),
            Err(_) => assert!(false)
        }
    }

}


