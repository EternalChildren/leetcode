pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    let mut sortStones = stones;
    sortStones.sort_by(|a, b| b.cmp(a));
    println!("{:?}", sortStones);

    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
    }
}
