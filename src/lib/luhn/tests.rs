#[cfg(test)]
mod tests {
    use crate::{sum_digits, toggle_multiplier};
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
