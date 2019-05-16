// 0ms 1.9MB
pub fn unique_paths_combination(m: i32, n: i32) -> i32 {
    let b = m + n - 2;
    let t = if m < n { m - 1 } else { n - 1 };
    let mut ans: i64 = 1;
    for i in 1..=t {
        ans = ans * (b - i + 1) as i64 / i as i64;
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(unique_paths_combination(3, 2), 3);
        assert_eq!(unique_paths_combination(7, 3), 28);
        assert_eq!(unique_paths_combination(51, 9), 1916797311);
    }
}
