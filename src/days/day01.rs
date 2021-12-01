use std::os::unix::prelude::MetadataExt;


pub fn part1(input: &str) -> Result<String, &str> {
    let inputs: Vec<i32> = input.lines()
        .map(|f| f.trim().parse::<i32>().unwrap())
        .collect();

    let mut result = 0;
    let mut previous_value: Option<i32> = None;
    inputs.iter()
        .for_each(|f| {
            if let Some(value) = previous_value {
                if f > &value {
                    result += 1;
                }
            }
            previous_value = Some(*f);
        });

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Result<String, &str> {
    let inputs: Vec<i32> = input.lines()
        .map(|f| f.trim().parse().unwrap())
        .collect();
    
    let mut result = 0;
    let mut previous_value: Option<i32> = None;
    for (i, value) in inputs.iter().enumerate() {
        // Check if we are able to create 3 pair
        if i+2 >= inputs.len() {
            break;
        }

        let sum = value + inputs[i+1] + inputs[i+2];
        if let Some(value) = previous_value {
            if sum > value {
                result += 1;
            }
        }

        previous_value = Some(sum);
    }

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


