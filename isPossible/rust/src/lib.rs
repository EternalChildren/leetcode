use std::collections::HashMap;

// 28ms 2.2MB
pub fn is_possible(nums: Vec<i32>) -> bool {
    let mut hash = HashMap::new();
    let mut end = HashMap::new();

    for v in nums.iter() {
        if let Some(k) = hash.get_mut(v) {
            *k += 1;
        } else {
            hash.insert(*v, 1);
        }
    }
    for v in nums.iter() {
        if let Some(k) = hash.get_mut(v) {
            if *k == 0 {
                continue;
            }
            *k -= 1;
        }
        if end.contains_key(&(v - 1)) && end.get(&(v - 1)).unwrap() > &0 {
            if let Some(i) = end.get_mut(&(v - 1)) {
                *i -= 1;
            }
            if let Some(i) = end.get_mut(v) {
                *i += 1;
            } else {
                end.insert(*v, 1);
            }
        } else if hash.contains_key(&(v + 1))
            && hash.contains_key(&(v + 2))
            && hash.get(&(v + 1)).unwrap() > &0
            && hash.get(&(v + 2)).unwrap() > &0
        {
            if let Some(i) = hash.get_mut(&(v + 1)) {
                *i -= 1;
            }
            if let Some(i) = hash.get_mut(&(v + 2)) {
                *i -= 1;
            }
            if let Some(m) = end.get_mut(&(v + 2)) {
                *m += 1;
            } else {
                end.insert(v + 2, 1);
            }
        } else {
            return false;
        }
        println!("end is {:?}", end);
    }
    println!("hash is {:?}", hash);
    println!("end is {:?}", end);

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let vec = vec![1, 2, 3, 3, 4, 5];
        assert_eq!(is_possible(vec), true);
    }

    #[test]
    fn second() {
        let vec1 = vec![1, 2, 3, 3, 4, 4, 5, 5];
        assert_eq!(is_possible(vec1), true);
    }

    #[test]
    fn thired() {
        let vec1 = vec![1, 2, 3, 4, 4, 5];
        assert_eq!(is_possible(vec1), false);
    }
}
