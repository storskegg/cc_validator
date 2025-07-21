use std::error::Error;
use std::str::Chars;

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

fn get_parity_digit(data: &str) -> Result<i32, ErrorBadInput> {
    let c = data.chars().last().unwrap();
    if c.is_digit(10) {
        Ok(c.to_digit(10).unwrap().cast_signed())
    } else {
        Err(ErrorBadInput)
    }
}

fn massage_input_string(data: &str) -> String {
    data.to_string().trim().get(..MAX_DATA).unwrap().to_string()
}

impl LuhnIntermediary {
    fn new(data: &str) -> Result<LuhnIntermediary, ErrorBadInput> {
        let incoming_str: String = massage_input_string(data);
        let local_data = incoming_str.chars();

        let pd = get_parity_digit(data);
        if pd.is_err() {
            return Err(ErrorBadInput);
        }

        let parity_digit = pd?;

        let mut digits: Vec<i32> = Vec::new();

        for (_, c) in local_data.rev().skip(1).enumerate() {
            let d = c.to_digit(10).unwrap().cast_signed();
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
