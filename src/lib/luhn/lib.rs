pub enum LuhnResult {
    Valid,
    Invalid,
    ErrBadInput,
}

// MAX_DATA is the maximum number of characters in the input string that we will handle
pub const MAX_DATA: usize = 64;

// ErrorBadInput is a custom error type for when we come across unsupported characters
pub struct ErrorBadInput;

impl std::fmt::Display for ErrorBadInput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Bad input: numerical string contains unsupported characters")
    }
}


// validate performs the complete Luhn validation process. Unlike the C++ implementation in
// this codebase, we handle this as a simple function, versus an instantiatable class.
pub fn validate(data: &str) -> Result<LuhnResult, ErrorBadInput> {
    let mut local_data: String = data.to_string();
    local_data.truncate(MAX_DATA);

    let i_res = LuhnIntermediary::new(local_data.as_str());
    if i_res.is_err() {
        return Err(i_res.err().unwrap())
    }

    let intermediary = i_res?;

    let mut current_multiplier = toggle_multiplier(0);
    let mut sum = 0;

    for (_, digit) in intermediary.digits.iter().enumerate() {
        sum += sum_digits(digit * current_multiplier);
        current_multiplier = toggle_multiplier(current_multiplier);
    }

    let lm = (10 - (sum % 10)) % 10;
    if lm != intermediary.parity_digit {
        return Ok(LuhnResult::Invalid);
    }

    Ok(LuhnResult::Valid)
}

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
    data.to_string().trim().get(..MAX_DATA).unwrap().to_string()
}


// LuhnIntermediary is a private struct that holds the numerical data and parity digit of the input
// string. The digits are stored in reverse order, allowing for conventional iteration when the
// summing process begins.
struct LuhnIntermediary {
    parity_digit: i32,
    digits: Vec<i32>,
    _data: String,
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
}

////////////////////////////////////////////////


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
