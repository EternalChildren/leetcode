pub fn sort_colors(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k = (n - 1) as i32;

    while (j as i32) <= k {
        if nums[j] == 1 {
            j += 1;
        } else if nums[j] == 0 {
            let temp = nums[j];
            nums[j] = nums[i];
            nums[i] = temp;
            j += 1;
            i += 1;
        } else {
            let temp = nums[j];
            nums[j] = nums[k as usize];
            nums[k as usize] = temp;
            k -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec: Vec<i32> = vec![2];
        let res: Vec<i32> = vec![2];
        sort_colors(&mut vec);
        assert_eq!(vec, res);
    }
}
