// 4ms 2.1MB
pub fn is_long_pressed_name(name: String, typed: String) -> bool {
    let vec = name.as_bytes();
    let mut index = 0;

    for k in typed.bytes() {
        println!("index is {}", index);
        if k == vec[index] {
            if index < vec.len() - 1 {
                index += 1;
            }
        } else {
            if index == 0 || vec[index - 1] != k {
                return false;
            }
        }
    }
    if vec[index] != typed.bytes().last().unwrap() {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let name = String::from("alex");
        let typed = String::from("aaleex");
        assert_eq!(is_long_pressed_name(name, typed), true);

        let name1 = String::from("saeed");
        let typed1 = String::from("ssaaedd");
        assert_eq!(is_long_pressed_name(name1, typed1), false);

        let name2 = String::from("leelee");
        let typed2 = String::from("lleeelee");
        assert_eq!(is_long_pressed_name(name2, typed2), true);

        let name3 = String::from("laiden");
        let typed3 = String::from("laiden");
        assert_eq!(is_long_pressed_name(name3, typed3), true);

        let name4 = String::from("vtkgn");
        let typed4 = String::from("vttkgnn");
        assert_eq!(is_long_pressed_name(name4, typed4), true);

        let name4 = String::from("vtkgn");
        let typed4 = String::from("vttkgnn");
        assert_eq!(is_long_pressed_name(name4, typed4), true);

        let name5 = String::from("dfuyalc");
        let typed5 = String::from("fuuyallc");
        assert_eq!(is_long_pressed_name(name5, typed5), false);
    }
}
