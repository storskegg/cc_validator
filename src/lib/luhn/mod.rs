mod tests;

use std::fmt;

pub enum LuhnResult {
    Valid,
    Invalid,
}

impl fmt::Display for LuhnResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LuhnResult::Valid => write!(f, "Valid"),
            LuhnResult::Invalid => write!(f, "Invalid"),
        }
    }
}

impl fmt::Debug for LuhnResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LuhnResult::Valid => write!(f, "[0] Valid Kuhn Checksum; File {}, Line {}", file!(), line!()),
            LuhnResult::Invalid => write!(f, "[1] Invalid Kuhn Checksum; File {}, Line {}", file!(), line!()),
        }
    }
}

// MAX_DATA is the maximum number of characters in the input string that we will handle
pub const MAX_DATA: usize = 64;

// ErrorBadInput is a custom error type for when we come across unsupported characters
pub struct ErrorBadInput;

impl fmt::Display for ErrorBadInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad input: numerical string contains unsupported characters")
    }
}

impl fmt::Debug for ErrorBadInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Input contains unsupported characters; File {}, Line {}", file!(), line!())
    }
}

pub fn calculate_modulus(data: &str) -> i32 {
    let intermediary = new_luhn_intermediary(data, false);
    intermediary.unwrap().calculate_modulus()
}

pub fn calculate_modulus_with_u128(data: u128) -> i32 {
    let intermediary = new_luhn_intermediary_with_u128(data, false);
    intermediary.unwrap().calculate_modulus()
}

pub fn validate_with_u64(data: u64) -> i32 {
    let intermediary = new_luhn_intermediary_with_u64(data, false);
    intermediary.unwrap().calculate_modulus()
}


// validate performs the complete Luhn validation process. Unlike the C++ implementation in
// this codebase, we handle this as a simple function, versus an instantiatable class.
pub fn validate(data: &str) -> Result<LuhnResult, ErrorBadInput> {
    // Let's keep mutability within the scope of the block, and not pass it on through the rest of
    // the function.
    let local_data: String = {
        let mut local_data: String = data.to_string();
        local_data.truncate(MAX_DATA);
        local_data
    };

    let intermediary = new_luhn_intermediary(local_data.as_str(), true)?;

    let lm: i32 = intermediary.calculate_modulus();

    println!("Calculated Luhn modulus: {}", lm);
    println!("Existing Luhn modulus: {}", intermediary.parity_digit);
    if lm != intermediary.parity_digit {
        return Ok(LuhnResult::Invalid);
    }

    Ok(LuhnResult::Valid)
}

// Private from here on

fn get_parity_digit(data: &str) -> Result<i32, ErrorBadInput> {
    let c = data.chars().last().unwrap();
    if c.is_digit(10) {
        Ok(c.to_digit(10).unwrap().cast_signed())
    } else {
        Err(ErrorBadInput)
    }
}

// massage_input_string trims the input string, truncates it to MAX_DATA, and returns the result as
// a string.
fn massage_input_string(data: &str) -> String {
    let mut tmp_trim: String = data.to_string().trim().to_string();
    tmp_trim.truncate(MAX_DATA);
    return tmp_trim.to_string()
}


// LuhnIntermediary is a private struct that holds the numerical data and parity digit of the input
// string. The digits are stored in reverse order, allowing for conventional iteration when the
// summing process begins.
struct LuhnIntermediary {
    pub parity_digit: i32,
    pub digits: Vec<i32>,
    _data: String,
}

impl LuhnIntermediary {
    fn new(data: &str, skip: bool) -> Result<LuhnIntermediary, ErrorBadInput> {
        let incoming_str: String = massage_input_string(data);
        let local_data = incoming_str.chars();

        let pd = get_parity_digit(data);
        if pd.is_err() {
            return Err(ErrorBadInput);
        }

        let parity_digit = pd?;

        let mut digits: Vec<i32> = Vec::new();

        let skip: usize = if skip { 1 } else { 0 };

        for (_, c) in local_data.rev().skip(skip).enumerate() {
            // ignore whitespace and hyphens
            if c.is_whitespace() || c == '-' {
                continue;
            }
            if !c.is_digit(10) {
                return Err(ErrorBadInput);
            }
            let d = c.to_digit(10).unwrap().cast_signed();
            digits.push(d);
        }

        Ok(LuhnIntermediary {
            parity_digit,
            digits,
            _data: incoming_str,
        })
    }

    pub fn calculate_modulus(&self) -> i32 {
        let mut sum = 0;
        let mut current_multiplier = toggle_multiplier(0);

        for (_, digit) in self.digits.iter().enumerate() {
            sum += sum_digits(digit * current_multiplier);
            current_multiplier = toggle_multiplier(current_multiplier);
        }

        (10 - (sum % 10)) % 10
    }
}

// new_luhn_intermediary is a private function that creates a new LuhnIntermediary from a given
// input string. It returns a Result containing the LuhnIntermediary, or an ErrorBadInput if the
// input string contains unsupported characters. It is functionally equivalent to
// LuhnIntermediary::new, but use this instead.
fn new_luhn_intermediary(data: &str, skip: bool) -> Result<LuhnIntermediary, ErrorBadInput> {
    LuhnIntermediary::new(data, skip)
}

fn new_luhn_intermediary_with_u64(data: u64, skip: bool) -> Result<LuhnIntermediary, ErrorBadInput> {
    let s: String = data.to_string();
    new_luhn_intermediary(&s, skip)
}

fn new_luhn_intermediary_with_u128(data: u128, skip: bool) -> Result<LuhnIntermediary, ErrorBadInput> {
    let s: String = data.to_string();
    new_luhn_intermediary(&s, skip)
}

// sum_digits calculates the sum of the digits in a given number. (e.g. 13 = 4)
fn sum_digits(n: i32) -> i32 {
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

// toggle_multiplier toggles the multiplier for the next digit.
fn toggle_multiplier(n: i32) -> i32 {
    match n {
        2 => 1,
        _ => 2,
    }
}
