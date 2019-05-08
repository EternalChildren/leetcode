// Given an array of 2n integers, your task is to group these integers into n pairs of integer, say (a1, b1), (a2, b2), ..., (an, bn) which makes sum of min(ai, bi) for all i from 1 to n as large as possible.


let nums = [1,2,3,2]

// 436ms
func arrayPairSum(_ nums: [Int]) -> Int {
    var res = 0
    let sorted = nums.sorted()
    for i in 0..<sorted.count {
        if i%2 == 0 {
            res += sorted[i]
        }
    }
    return res
}

arrayPairSum(nums)
