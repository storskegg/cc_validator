use std::error::Error;

pub enum LuhnResult {
    Valid,
    Invalid,
    ErrBadInput,
}

pub const MAX_DATA: usize = 64;

// TODO: Return Result<LuhnResult, ErrType>
pub fn validate(data: &str) -> LuhnResult {
    let mut local_data: String = data.to_string();
    local_data.truncate(MAX_DATA);

    let mut parity_digit = -1;
    let mut digits: Vec<i32> = {
        let mut v = Vec::with_capacity(local_data.len()-1);

    }

    let mut last_multiplier = 1;
    let mut sum = 0;

    for (i, c) in d.chars().rev().enumerate() {
        if c.is_whitespace() || c == '-' {
            continue;
        }

        let digit = c.to_digit(10);
        if digit.is_none() {
            return LuhnResult::ErrBadInput;
        }
    }

    LuhnResult::Valid
}

struct ErrorBadInput;

impl std::fmt::Display for ErrorBadInput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Bad input: numerical string contains unsupported characters")
    }
}

struct LuhnIntermediary {
    parity_digit: i32,
    digits: Vec<i32>,
}

impl LuhnIntermediary {
    fn new(data: &str) -> Result<LuhnIntermediary, ErrorBadInput> {
        let local_data = data.to_string().trim().get(..MAX_DATA).unwrap().chars();
        let c = local_data.last().unwrap();
        let parity_digit = {
            // let c = local_data.last().unwrap();
            if c.is_digit(10) {
                c.to_digit(10).unwrap()
            } else {
                return Err(ErrorBadInput);
            }
        };

        let mut digits = Vec::new();

        for (_, c) in local_data.rev().skip(1).enumerate() {
            let d = c.to_digit(10).unwrap();
            digits.push(d);
        }

        Ok(LuhnIntermediary {
            parity_digit,
            digits,
        })
    }
}

fn vec_from_str(s: &str) -> Result<LuhnIntermediary, ErrorBadInput> {
    let parity_digit = -1;
    let mut v = Vec::with_capacity(MAX_DATA - 1);




    for (i, c) in s.chars().rev().enumerate() {
        if c.is_whitespace() || c == '-' {
            continue;
        }

        let digit = c.to_digit(10);
        if digit.is_none() {
            return Err(ErrorBadInput);
        }

        v.push(digit.unwrap());
    }

    return Ok(LuhnIntermediary {
        parity_digit,
        digits: v,
    })
}



fn sum_digits(n: i32) -> i32 {
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn toggle_multiplier(n: i32) -> i32 {
    match n {
        2 => 1,
        _ => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_digits_given_one_digit() {
        let result = sum_digits(5);
        assert_eq!(result, 5);
    }

    #[test]
    fn sum_digits_given_two_digits() {
        let result = sum_digits(54);
        assert_eq!(result, 9);
    }
    #[test]
    fn sum_digits_given_three_digits() {
        let result = sum_digits(543);
        assert_eq!(result, 12);
    }

    #[test]
    fn sum_digits_given_four_digits() {
        let result = sum_digits(5432);
        assert_eq!(result, 14);
    }

    #[test]
    fn sum_digits_given_five_digits() {
        let result = sum_digits(54321);
        assert_eq!(result, 15);
    }

    #[test]
    fn toggle_multiplier_given_zero() {
        let result = toggle_multiplier(0);
        assert_eq!(result, 2);
    }

    #[test]
    fn toggle_multiplier_given_one() {
        let result = toggle_multiplier(1);
        assert_eq!(result, 2);
    }

    #[test]
    fn toggle_multiplier_given_two() {
        let result = toggle_multiplier(2);
        assert_eq!(result, 1);
    }

    #[test]
    fn toggle_multiplier_given_three() {
        let result = toggle_multiplier(3);
        assert_eq!(result, 2);
    }

    #[test]
    fn toggle_multiplier_given_negative_one() {
        let result = toggle_multiplier(-1);
        assert_eq!(result, 2);
    }
}
