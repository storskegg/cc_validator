const NUM_DIGITS: u128 = 10;
const QTY_NUMBERS: u128 = 50;

fn main() {
    let first_digit: u128 = 4;
    let mult: u128 = {
        let mut result: u128 = 1;
        for _ in 0..(NUM_DIGITS - 2) {
            result *= 10;
        }
        result
    };

    let start_number = first_digit * (mult);

    println!("Start   = '{: >16}'", start_number);

    for i in 0..QTY_NUMBERS {
        println!("i       = '{: >16}'", i);
        let from = start_number + i;
        println!("from    = '{: >16}'", from);
        let modulus = luhn::calculate_modulus_with_u128(from).cast_unsigned();
        println!("modulus = '{: >16}'", modulus);
        let num = (from * 10) + u128::from(modulus);
        println!("num     = '{: >16}'", num);
    }
}
