use std::cmp::Ordering;


/// Compare two `f64` values, with an absolute tolerance.
pub fn compare_floats<T1, T2, T3>(lhs: T1, rhs: T2, tolerance: T3)
    -> Option<Ordering>
    where T1: Into<f64>,
          T2: Into<f64>,
          T3: Into<f64> {
    let lhs: f64 = lhs.into();
    let rhs: f64 = rhs.into();
    let tolerance: f64 = tolerance.into();

    if tolerance.is_nan() {
        panic!("tolerance cannot be NaN.");
    }
    let tolerance = tolerance.abs();

    // If both numbers are `NaN`, the result should be `None`.
    if lhs.is_nan() || rhs.is_nan() {
        return None;
    }

    // If both numbers are infinite and the same sign, the result should be
    // `Ordering::Equal`.
    if lhs.is_infinite() && rhs.is_infinite() && 
        lhs.is_sign_positive() == rhs.is_sign_positive() {
        return Some(Ordering::Equal);
    }

    let diff = lhs - rhs;   // `diff` cannot be `NaN`.
    if diff.abs() <= tolerance {
        return Some(Ordering::Equal);
    }

    if diff < 0f64 {
        Some(Ordering::Less)
    } else {
        Some(Ordering::Greater)
    }
}


pub mod preclude {
    pub use super::compare_floats;
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::compare_floats;

    #[test]
    fn test_compare_floats() {
        assert_eq!(compare_floats(1.0, 1.0, 1e-6), Some(Ordering::Equal));
        assert_eq!(compare_floats(1.0, 2.0, 1e-6), Some(Ordering::Less));
        assert_eq!(compare_floats(2.0, 1.0, 1e-6), Some(Ordering::Greater));
        assert_eq!(compare_floats(1.0, 1.0 + 1e-8, 1e-6), Some(Ordering::Equal));
        assert_eq!(compare_floats(1.0, 1.0 + 1e-8, -1e-6), Some(Ordering::Equal));
        assert_eq!(compare_floats(1.0, 1.0 - 1e-8, 1e-6), Some(Ordering::Equal));
        assert_eq!(compare_floats(1.0, std::f64::NAN, 1e-6), None);
        assert_eq!(compare_floats(std::f64::NAN, 1.0, 1e-6), None);
        assert_eq!(compare_floats(std::f64::NAN, std::f64::NAN, 1e-6), None);
        assert_eq!(compare_floats(std::f64::INFINITY, 1.0, 1e-6), Some(Ordering::Greater));
        assert_eq!(compare_floats(1.0, std::f64::INFINITY, 1e-6), Some(Ordering::Less));
        assert_eq!(compare_floats(std::f64::INFINITY, std::f64::INFINITY, 1e-6), 
            Some(Ordering::Equal));
        assert_eq!(compare_floats(std::f64::NEG_INFINITY, 1.0, 1e-6), Some(Ordering::Less));
        assert_eq!(compare_floats(1.0, std::f64::NEG_INFINITY, 1e-6), Some(Ordering::Greater));
        assert_eq!(compare_floats(std::f64::NEG_INFINITY, std::f64::NEG_INFINITY, 1e-6), 
            Some(Ordering::Equal));
        assert_eq!(compare_floats(std::f64::INFINITY, std::f64::NEG_INFINITY, 1e-6), 
            Some(Ordering::Greater));
        assert_eq!(compare_floats(std::f64::NEG_INFINITY, std::f64::INFINITY, 1e-6), 
            Some(Ordering::Less));
    }
}
