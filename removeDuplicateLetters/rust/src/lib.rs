use std::collections::HashMap;

// 0ms 2MB
pub fn remove_duplicate_letters(s: String) -> String {
    let mut hash = HashMap::new();
    let mut res = String::new();

    for c in s.chars() {
        if let Some(k) = hash.get(&c) {
            hash.insert(c, k + 1);
        } else {
            hash.insert(c, 1);
        }
    }

    for c in s.chars() {
        if res.find(c) == None {
            while !res.is_empty()
                && hash.get(&res.chars().last().unwrap()).unwrap() != &0
                && res.chars().last().unwrap() > c
            {
                res.pop();
            }
            println!("res: {:?}", res);
            res.push(c);
        }
        hash.insert(c, hash.get(&c).unwrap() - 1);
    }

    println!("res is {:?}", res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "bcabc".to_string();
        assert_eq!(remove_duplicate_letters(s), "abc".to_string());
        let s1 = "cbacdcbc".to_string();
        assert_eq!(remove_duplicate_letters(s1), "acdb".to_string());
        let s2 = "edebbed".to_string();
        assert_eq!(remove_duplicate_letters(s2), "bed".to_string());
    }
}
