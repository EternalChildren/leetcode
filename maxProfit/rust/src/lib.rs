// 0ms 2.2MB
pub fn max_profit_fastly(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }
    let mut ans = 0;
    let mut min = std::i32::MAX;

    for p in prices {
        let profit = std::cmp::max(p - min, 0);
        if profit > ans {
            ans = profit;
        }
        min = std::cmp::min(min, p);
    }
    ans
}

// 72ms 2.3MB
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }
    let mut ans = 0;

    for i in 0..prices.len() {
        for j in i + 1..prices.len() {
            let dif = prices[j] - prices[i];
            if dif > ans {
                ans = dif;
            }
        }
    }
    println!("res is {}", ans);
    ans
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(vec), 5);
        let vec1 = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(vec1), 0);
    }

    #[test]
    fn quickly() {
        let vec = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit_fastly(vec), 5);
        let vec1 = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit_fastly(vec1), 0);
    }
}
