// 0ms 2MB
pub fn climb_stairs(n: i32) -> i32 {
    match n {
        1 => 1,
        2 => 2,
        k => {
            let mut i = 1;
            let mut j = 2;
            let mut n = k - 2;
            while n > 0 {
                let k = i;
                i = j;
                j = k + j;
                n -= 1;
            }
            j
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
    }
}
