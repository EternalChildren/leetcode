// 0ms 2.2MB
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }
    let mut ans = 0;
    for i in 0..prices.len() - 1 {
        if prices[i] < prices[i + 1] {
            ans += prices[i + 1] - prices[i];
        }
    }
    println!("ans is {}", ans);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(vec), 7);
        let vec1 = vec![1, 2, 3, 4, 5];
        assert_eq!(max_profit(vec1), 4);
        let vec2 = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(vec2), 0);
    }
}
