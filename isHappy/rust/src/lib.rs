// 0ms 2MB
pub fn is_happy(n: i32) -> bool {
    let mut i = n;
    while i != 1 && i != 4 {
        let mut sum = 0;
        while i > 0 {
            let m = i % 10;
            sum += m * m;
            i = i / 10;
        }
        i = sum;
        println!("sum is {}", sum);
    }

    i == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_happy(4), false);
        assert_eq!(is_happy(19), true);
    }
}
