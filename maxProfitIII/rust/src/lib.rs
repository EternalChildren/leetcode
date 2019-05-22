// 0ms 2.2MB 大佬代码 暂时还不太明白
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit_1 = 0;
    let mut max_profit_2 = 0;
    let mut lowest_buy_price_1 = std::i32::MIN;
    let mut lowest_buy_price_2 = std::i32::MIN;

    for p in prices {
        lowest_buy_price_1 = std::cmp::max(lowest_buy_price_1, -p);
        max_profit_1 = std::cmp::max(max_profit_1, lowest_buy_price_1 + p);
        lowest_buy_price_2 = std::cmp::max(lowest_buy_price_2, max_profit_1 - p);
        max_profit_2 = std::cmp::max(max_profit_2, p + lowest_buy_price_2);
    }

    max_profit_2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec = vec![3, 3, 5, 0, 0, 3, 1, 4];
        assert_eq!(max_profit(vec), 6);
        let vec1 = vec![1, 2, 3, 4, 5];
        assert_eq!(max_profit(vec1), 4);
        let vec2 = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(vec2), 0);
    }

    #[test]
    fn last() {
        let vec = vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0];
        assert_eq!(max_profit(vec), 13);
    }
}
