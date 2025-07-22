// insert a file comment here

use std::io::{self};
use luhn::{self};

fn main() {
    let mut buf = String::with_capacity(luhn::MAX_DATA);
    println!("Enter a string to be validated:");
    let _ = io::stdin().read_line(&mut buf).unwrap();
    buf = buf.trim().to_string();
    let num_read = buf.chars().count();

    println!("Digits: {}", buf.trim());
    println!("Number of digits: {}", num_read);

    let vr = luhn::validate(&buf);
    if vr.is_err() {
        println!("Input contains invalid characters.");
        return;
    }
    match vr.unwrap() {
        luhn::LuhnResult::Valid => println!("Input is valid."),
        luhn::LuhnResult::Invalid => println!("Input is not valid."),
    }
}
