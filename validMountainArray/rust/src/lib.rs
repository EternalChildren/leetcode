// 4ms 2.2MB
pub fn valid_mountain_array(a: Vec<i32>) -> bool {
    if a.len() < 3 {
        return false;
    }
    let mut sign = false;
    let mut num = -1;

    for (i, v) in a.iter().enumerate() {
        println!("sign is {}", sign);
        println!("num is {}", num);
        if !sign {
            if v > &num {
                num = *v;
            } else if v < &num {
                if i == 1 {
                    return false;
                }
                sign = true;
                num = *v;
            } else {
                return false;
            }
        } else {
            if v < &num {
                num = *v;
            } else {
                return false;
            }
        }
    }

    if !sign {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![2, 1];
        assert_eq!(valid_mountain_array(v), false);

        let v1 = vec![3, 5, 5];
        assert_eq!(valid_mountain_array(v1), false);

        let v2 = vec![0, 3, 2, 1];
        assert_eq!(valid_mountain_array(v2), true);
    }

    #[test]
    fn special() {
        let v3 = vec![9, 8, 7, 6, 0];
        assert_eq!(valid_mountain_array(v3), false);

        let v4 = vec![0, 7, 8, 9];
        assert_eq!(valid_mountain_array(v4), false);
    }

    #[test]
    fn last() {
        let v = vec![3, 7, 20, 14, 15, 14, 10, 8, 2, 1];
        assert_eq!(valid_mountain_array(v), false);
    }
}
