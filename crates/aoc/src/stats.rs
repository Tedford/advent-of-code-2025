#[allow(dead_code)]
pub fn combinations_with_repetition(n: usize, r: usize) -> usize {
    fn factorial(num: usize) -> usize {
        (1..=num).product()
    }

    factorial(n + r - 1) / (factorial(r) * factorial(n - 1))
}

#[allow(dead_code)]
pub fn combinations(n: usize, r: usize) -> usize {
    fn factorial(num: usize) -> usize {
        (1..=num).product()
    }

    factorial(n) / (factorial(r) * factorial(n - r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combinations_with_repetition_valid () {
        assert_eq!(combinations_with_repetition(3, 6), 28);
    }

    #[test]
    fn combinations_valid () {
         assert_eq!(combinations(5, 3), 10);
    }
}
