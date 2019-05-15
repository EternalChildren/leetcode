use std::collections::HashMap;

// 8ms 3MB
pub fn find_relative_ranks_hash(nums: Vec<i32>) -> Vec<String> {
    let mut sorted = nums.to_vec();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut hash = HashMap::new();
    let mut res = vec![nums.len().to_string(); nums.len()];

    for (i, v) in sorted.iter().enumerate() {
        match i {
            0 => {
                hash.insert(v, "Gold Medal".to_string());
            }
            1 => {
                hash.insert(v, "Silver Medal".to_string());
            }
            2 => {
                hash.insert(v, "Bronze Medal".to_string());
            }
            _ => {
                hash.insert(v, (i + 1).to_string());
            }
        }
    }

    for (i, v) in nums.iter().enumerate() {
        if let Some(s) = hash.get(v) {
            res[i] = s.to_string();
        }
    }

    println!("res is {:?}", res);
    res
}

// 36ms 2.4MB
pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
    let mut sorted = nums.to_vec();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let mut res = vec![nums.len().to_string(); nums.len()];

    for (i, v) in sorted.iter().enumerate() {
        for (j, k) in nums.iter().enumerate() {
            if v == k {
                match i {
                    0 => {
                        res[j] = "Gold Medal".to_string();
                    }
                    1 => {
                        res[j] = "Silver Medal".to_string();
                    }
                    2 => {
                        res[j] = "Bronze Medal".to_string();
                    }
                    _ => {
                        res[j] = (i + 1).to_string();
                    }
                }
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec = vec![5, 4, 3, 2, 1];
        assert_eq!(
            find_relative_ranks(vec),
            vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
        );

        let vec1 = vec![5, 4, 3, 2, 1];
        assert_eq!(
            find_relative_ranks_hash(vec1),
            vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
        );
    }
}
