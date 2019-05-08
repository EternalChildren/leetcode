// Given an array of integers A sorted in non-decreasing order, return an array of the squares of each number, also in sorted non-decreasing order.

var nums = [-4,-3,0,3,10]

// 384ms
func sortedSquares1(_ A: [Int]) -> [Int] {
    var res = [Int]()
    for item in A {
        res.append(item * item)
    }
    res.sort()
    return res
}

// 372ms
func sortedSquares2(_ A: [Int]) -> [Int] {
    var res = [Int]()
    var l = 0
    var r = A.count - 1
    while l <= r {
        let left = A[l] * A[l]
        let right = A[r] * A[r]
        if left < right {
            res.insert(right, at: 0)
            r -= 1
        } else {
            res.insert(left, at: 0)
            l += 1
        }
    }
    
    return res
}

sortedSquares1(nums)

sortedSquares2(nums)
