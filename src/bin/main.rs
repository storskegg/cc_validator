// insert a file comment here

use std::io::{self};
use luhn::{self};

fn main() {
    let mut buf = String::with_capacity(luhn::MAX_DATA);
    println!("Enter a credit card number:");
    let _ = io::stdin().read_line(&mut buf).unwrap();
    buf = buf.trim().to_string();
    let num_read = buf.chars().count();

    println!("You entered [{}]: '{}'", num_read, buf.trim());

    let v1str = "2345";
    let v1r = luhn::validate(v1str);
    if v1r.is_err() {
        println!("Error: {}", v1r.err().unwrap());
        return;
    }
    println!("Result ['{}']: {}", v1str, v1r.unwrap());

    let v2str = "12345";
    let v2r = luhn::validate(v2str);
    if v2r.is_err() {
        println!("Error: {}", v2r.err().unwrap());
        return;
    }
    println!("Result ['{}']: {}", v2str, v2r.unwrap());
}
