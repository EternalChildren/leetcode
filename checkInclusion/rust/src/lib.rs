use std::collections::HashMap;

// 超时
pub fn check_inclusion(s1: String, s2: String) -> bool {
    let mut hash = HashMap::new();

    for s in s1.chars() {
        match hash.get(&s) {
            Some(&i) => hash.insert(s, i + 1),
            None => hash.insert(s, 1),
        };
    }
    println!("hash is {:?}", hash);

    for (i, s) in s2.chars().enumerate() {
        let mut tmp = hash.clone();
        match tmp.get(&s) {
            Some(&num) => {
                let num = num - 1;
                if num == 0 {
                    tmp.remove(&s);
                } else {
                    tmp.insert(s, num);
                }
                if tmp.is_empty() {
                    return true;
                }

                for k in String::from(&s2[i + 1..]).chars() {
                    match tmp.get(&k) {
                        Some(&num) => {
                            let num = num - 1;
                            if num == 0 {
                                tmp.remove(&k);
                            } else {
                                tmp.insert(k, num);
                            }
                            if tmp.is_empty() {
                                return true;
                            }
                        }
                        None => break,
                    }
                }
            }
            None => continue,
        }
    }

    false
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        let s1 = String::from("ab");
        let s2 = String::from("eidbaooo");
        assert_eq!(check_inclusion(s1, s2), true);
    }

    #[test]
    fn second() {
        let s1 = String::from("ab");
        let s2 = String::from("eidboaoo");
        assert_eq!(check_inclusion(s1, s2), false);
    }

    #[test]
    fn third() {
        let s1 = String::from("a");
        let s2 = String::from("ab");
        assert_eq!(check_inclusion(s1, s2), true);
    }

    #[test]
    fn fourth() {
        let s1 = String::from("abc");
        let s2 = String::from("ccccbbbbaaaa");
        assert_eq!(check_inclusion(s1, s2), false);
    }
}
