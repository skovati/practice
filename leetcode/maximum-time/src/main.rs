use std::env;

fn main() {
    let input = env::args().nth(1).unwrap();
    println!("{}", find_max(input));
}

fn find_max(input: String) -> String  {
    let mut output: Vec<char> = input.chars().collect();
    if output[0] == '?' {
        output[0] = if output[1] < '4' || output[1] == '?' {
            '2'
        } else {
            '1'
        }
    }
    if output[1] == '?' {
        output[1] = if output[0] == '2' {
            '3'
        } else {
            '9'
        }
    }
    if output[3] == '?' {
        output[3] = '5'
    }
    if output[4] == '?' {
        output[4] = '9'
    }
    return output.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_max("?4:5?".to_string()), "14:59");
    }

    #[test]
    fn test_2() {
        assert_eq!(find_max("23:5?".to_string()), "23:59");
    }

    #[test]
    fn test_3() {
        assert_eq!(find_max("2?:22".to_string()), "23:22");
    }

    #[test]
    fn test_4() {
        assert_eq!(find_max("0?:??".to_string()), "09:59");
    }

    #[test]
    fn test_5() {
        assert_eq!(find_max("??:??".to_string()), "23:59");
    }
}
