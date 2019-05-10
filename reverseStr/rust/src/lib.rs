// 0ms 2MB
pub fn reverse_str(s: String, k: i32) -> String {
    if s.len() == 0 {
        return String::new();
    }
    let k = k as usize;
    let mut tmp = vec![];
    let mut res = String::new();

    for ca in s.chars() {
        tmp.push(ca);

        if tmp.len() == 2 * k {
            let insert_position = res.len();
            for j in 0..tmp.len() {
                if j < k {
                    res.insert(insert_position, tmp[j]);
                } else {
                    res.push(tmp[j]);
                }
            }
            tmp.clear();
        }
    }

    if !tmp.is_empty() {
        let insert_position = res.len();
        for j in 0..tmp.len() {
            if j < k {
                res.insert(insert_position, tmp[j]);
            } else {
                res.push(tmp[j]);
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
        let s = String::from("abcdefg");

        assert_eq!(reverse_str(s, 2), "bacdfeg".to_string());
    }
}
