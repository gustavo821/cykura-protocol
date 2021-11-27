/// Math functions that do not check inputs or outputs
/// Contains methods that perform common math functions but do not do any
/// overflow or underflow checks

/// Returns ceil (x / y)
/// Division by 0 throws a panic, and must be checked externally
///
/// In Solidity dividing by 0 results in 0, not an exception.
///
/// # Arguments
///
/// * `x` - The dividend
/// * `y` - The divisor
///
pub fn div_rounding_up(x: u64, y: u64) -> u64 {
    x / y + ((x % y > 0) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_by_factor() {
        assert_eq!(div_rounding_up(4, 2), 2);
    }

    #[test]
    fn divide_and_round_up() {
        assert_eq!(div_rounding_up(4, 3), 2);
    }

    #[test]
    #[should_panic]
    fn divide_by_zero() {
        div_rounding_up(2, 0);
    }
}
