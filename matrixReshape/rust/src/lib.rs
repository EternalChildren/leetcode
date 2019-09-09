// 8ms 2.3mb
pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    if nums.len() * nums[0].len() != (r * c) as usize {
        return nums;
    }

    let mut ans = vec![];
    let mut tmp = vec![];

    for num in nums.iter() {
        for &i in num {
            tmp.push(i);
            println!("tmp is {:?}", tmp);
            if tmp.len() == c as usize {
                ans.push(tmp.clone());
                tmp.clear();
                println!("tmp is {:?}", tmp);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(matrix_reshape(nums.clone(), 1, 4), vec![vec![1, 2, 3, 4]]);

        assert_eq!(matrix_reshape(nums.clone(), 2, 4), nums.clone());
    }
}
