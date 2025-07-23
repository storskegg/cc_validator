const NUM_DIGITS: u16 = 5;
const QTY_NUMBERS: u64 = 50;

fn main() {
    let first_digit: u64 = 4;

    let mult: u64 = derive_multiplier();
    let start_number: u64 = first_digit * (mult);
    if QTY_NUMBERS > (u64::MAX - start_number) {
        eprintln!("Error: QTY_NUMBERS is too large. QTY_NUMBERS {} Start {} Last {} Max_Supported {}", QTY_NUMBERS, start_number, u128::from(start_number+QTY_NUMBERS), u64::MAX);
        return;
    }

    let mut last_number: u64 = 0;
    for i in 0..QTY_NUMBERS {
        // learned dynamic padding here: https://stackoverflow.com/questions/69067436/how-do-i-make-the-fill-padding-in-stdformat-dynamic
        let from: u64 = u64::from(start_number + i);
        let modulus: u32 = luhn::calculate_modulus_with_u64(from).cast_unsigned();
        let num: u64 = (from * 10) + u64::from(modulus);
        // println!("num   = '{: >width$}'", num, width = usize::from(NUM_DIGITS));
        let delta: u64 = calculate_delta(last_number, num);
        // println!("delta = '{: >width$}'", delta, width = usize::from(NUM_DIGITS));
        println!("(num, delta) = '{: >width$?}'", (num, delta), width = usize::from(NUM_DIGITS));

        last_number = num;
    }
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