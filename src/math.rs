pub fn lerp(a: f64, b: f64, alpha: f64) -> f64 {
    a * (1.0 - alpha) + b * alpha
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(1.0, 11.0, 0.2), 3.0);
    }
}
