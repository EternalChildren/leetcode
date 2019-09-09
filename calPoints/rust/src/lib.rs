// 0ms 2.1mb
pub fn cal_points(ops: Vec<String>) -> i32 {
    let mut sum = Vec::new();

    for it in ops.iter() {
        match &it[..] {
            "+" => {
                if sum.len() >= 2 {
                    let n = sum[sum.len() - 1] + sum[sum.len() - 2];
                    sum.push(n)
                }
            }
            "D" => {
                if let Some(&n) = sum.last() {
                    sum.push(n * 2);
                }
            }
            "C" => {
                sum.pop();
            }
            k => {
                let n: i32 = k.parse::<i32>().unwrap();
                sum.push(n);
            }
        }
    }

    sum.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            cal_points(vec![
                String::from("5"),
                String::from("2"),
                String::from("C"),
                String::from("D"),
                String::from("+"),
            ]),
            30
        )
    }
}
