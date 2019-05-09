use std::collections::HashMap;

// 20ms 2.1MB
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut hash = HashMap::new();

    for c in magazine.into_bytes() {
        match hash.get(&c) {
            Some(exist) => {
                hash.insert(c, exist + 1);
            }
            None => {
                hash.insert(c, 1);
            }
        }
    }

    for c in ransom_note.into_bytes() {
        match hash.get(&c) {
            Some(&exist) => {
                if exist == 0 {
                    return false;
                }
                hash.insert(c, exist - 1);
            }
            None => return false,
        }
    }
    println!("hash is {:?}", hash);
    true
}

// 0ms 2MB
// 数据遍历量会减少的时候,使用二重遍历暴力解,可能会比两次遍历速度要快。
pub fn can_construct_easy(ransom_note: String, magazine: String) -> bool {
    let ransom_note = ransom_note.into_bytes();
    let mut magazine = magazine.into_bytes();
    for c in ransom_note {
        let mut find = false;
        for (i, &v) in magazine.iter().enumerate() {
            if c == v {
                find = true;
                magazine.remove(i);
                break;
            }
        }
        if !find {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            can_construct(String::from("aaa"), String::from("aab")),
            false
        );
        assert_eq!(can_construct(String::from("a"), String::from("b")), false);
        assert_eq!(can_construct(String::from("aa"), String::from("ab")), false);
        assert_eq!(can_construct(String::from("aa"), String::from("aab")), true);

        assert_eq!(
            can_construct_easy(String::from("a"), String::from("b")),
            false
        );
        assert_eq!(
            can_construct_easy(String::from("aa"), String::from("ab")),
            false
        );
        assert_eq!(
            can_construct_easy(String::from("aa"), String::from("aab")),
            true
        );
    }
}
