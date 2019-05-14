// 28ms 2.2MB
pub fn next_greater_elements_stack(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut ans = vec![-1; len];
    let mut stack: Vec<i32> = vec![];

    for i in 0..nums.len() * 2 {
        let current = nums[i % len];
        while let Some(&k) = stack.last() {
            if nums[k as usize] < current {
                if let Some(p) = stack.pop() {
                    ans[p as usize] = current;
                }
            } else {
                break;
            }
        }

        if i < len {
            stack.push(i as i32);
        }
    }

    ans
}

// 40ms 2.2MB
pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[j] > nums[i] {
                ans.push(nums[j]);
                break;
            }
        }
        if ans.len() < i + 1 {
            for m in 0..i + 1 {
                if nums[m] > nums[i] {
                    ans.push(nums[m]);
                    break;
                }
            }
            if ans.len() < i + 1 {
                ans.push(-1);
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
        let input = vec![1, 2, 1];
        let input_stack = input.clone();

        assert_eq!(next_greater_elements(input), vec![2, -1, 2]);
        assert_eq!(next_greater_elements_stack(input_stack), vec![2, -1, 2]);

        let input1 = vec![1, 2, 3, 4, 3];
        let input1_stack = input1.clone();
        assert_eq!(next_greater_elements(input1), vec![2, 3, 4, -1, 4]);
        assert_eq!(
            next_greater_elements_stack(input1_stack),
            vec![2, 3, 4, -1, 4]
        );

        let input2 = vec![5, 4, 3, 2, 1];
        let input2_stack = input2.clone();
        assert_eq!(next_greater_elements(input2), vec![-1, 5, 5, 5, 5]);
        assert_eq!(
            next_greater_elements_stack(input2_stack),
            vec![-1, 5, 5, 5, 5]
        );

        let input3 = vec![100, 1, 11, 1, 120, 111, 123, 1, -1, -100];
        assert_eq!(
            next_greater_elements(input3),
            vec![120, 11, 120, 120, 123, 123, -1, 100, 100, 100]
        );

        let input3 = vec![100, 1, 11, 1, 120, 111, 123, 1, -1, -100];
        let input3_stack = input3.clone();
        assert_eq!(
            next_greater_elements_stack(input3_stack),
            vec![120, 11, 120, 120, 123, 123, -1, 100, 100, 100]
        );
    }
}
