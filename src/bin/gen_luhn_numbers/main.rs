const NUM_DIGITS: u16 = 10;
const QTY_NUMBERS: u16 = 50;

fn main() {
    let first_digit: u16 = 4;

    // TODO: break the block into a function for testability and readability
    let mult: u16 = {
        // keep mutability localized within the scope of the block.
        let mut result: u16 = 1;
        for _ in 0..(NUM_DIGITS - 2) {
            result *= 10;
        }
        result
    };

    let start_number: u16 = first_digit * (mult);

    println!("Start   = '{: >16}'", start_number);

    for i in 0..QTY_NUMBERS {
        // learned dynamic padding here: https://stackoverflow.com/questions/69067436/how-do-i-make-the-fill-padding-in-stdformat-dynamic
        let from: u64 = u64::from(start_number + i);
        let modulus: u32 = luhn::calculate_modulus_with_u64(from).cast_unsigned();
        let num: u64 = (from * 10) + u64::from(modulus);
        println!("num = '{: ^width$}'", num, width = usize::from(NUM_DIGITS));
    }
}
