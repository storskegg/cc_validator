use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

const NUM_DIGITS: u16 = 16;
const QTY_NUMBERS: u64 = 999999999;


fn main() -> Result<(), Box<dyn Error>> {
    let first_digit: u64 = 4;

    let start_number: u64 = first_digit * derive_multiplier();
    if QTY_NUMBERS > (u64::MAX - start_number) {
        eprintln!("Error: QTY_NUMBERS is too large. QTY_NUMBERS {} Start {} Last {} Max_Supported {}", QTY_NUMBERS, start_number, u128::from(start_number+QTY_NUMBERS), u64::MAX);
        return Ok(()); // TODO: <-- This is wrong, just moving on elsewhere at the moment.
    }

    let mut out = io::BufWriter::new(File::create("./out.csv")?);

    let mut last_number: u64 = 0;
    print!("Progress: {:>5.1}%", 0.0);
    for i in 0..QTY_NUMBERS {
        // learned dynamic padding here: https://stackoverflow.com/questions/69067436/how-do-i-make-the-fill-padding-in-stdformat-dynamic
        let from: u64 = start_number + i;
        let modulus: u32 = luhn::calculate_modulus_with_u64(from);
        let num: u64 = (from * 10) + u64::from(modulus);
        // println!("num   = '{: >width$}'", num, width = usize::from(NUM_DIGITS));
        let delta: u64 = calculate_delta(last_number, num);
        // println!("delta = '{: >width$}'", delta, width = usize::from(NUM_DIGITS));
        // println!("(num, delta) = '{: >width$?}'", (num, delta), width = usize::from(NUM_DIGITS));
        let _ = writeln!(out, "{}, {}", num, delta);
        last_number = num;
        if i % (QTY_NUMBERS / 1000) == 0 {
            print!("\r\x1b[2KProgress: {:>5.1}% -- {}", (i as f64 / QTY_NUMBERS as f64) * 100.0, last_number);
            io::stdout().flush().unwrap();
        }
    }

    out.flush()?;

    println!("\r\x1b[2KProgress: {:>5.1}% -- {}", 100.0, last_number);

    Ok(())
}

fn calculate_delta(last: u64, current: u64) -> u64 {
    if last == 0 {
        return last;
    }
    current - last
}

fn derive_multiplier() -> u64 {
    let mut result: u64 = 1;
    for _ in 0..(NUM_DIGITS - 2) {
        result *= 10;
    }
    result
}
