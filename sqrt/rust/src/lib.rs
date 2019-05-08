// 4ms 1.9MB
pub fn my_sqrt(x: i32) -> i32 {
    let x: f64 = x.into();
    let mut target = x;

    if x < 2.0 {
        return x as i32;
    }

    while target * target - x > 0.00001 {
        target = target / 2.0 + x / (2.0 * target);
    }
    target as i32
}

// 4ms 2.2MB
pub fn system_sqrt(x: i32) -> i32 {
    (x as f64).sqrt().floor() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sqrt() {
        assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(0), 0);
        assert_eq!(my_sqrt(2), 1);
        assert_eq!(my_sqrt(1), 1);
    }

    #[test]
    fn sqrt_system() {
        assert_eq!(system_sqrt(4), 2);
        assert_eq!(system_sqrt(0), 0);
        assert_eq!(system_sqrt(2), 1);
        assert_eq!(system_sqrt(1), 1);
    }

}
