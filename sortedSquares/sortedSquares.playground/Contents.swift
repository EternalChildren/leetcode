// Given an array of integers A sorted in non-decreasing order, return an array of the squares of each number, also in sorted non-decreasing order.


var nums = [-4,-3,0,3,10]

func sortedSquares(_ A: [Int]) -> [Int] {
    var res = [Int]()
    for item in A {
        res.append(item * item)
    }
    res.sort()
    return res
}

sortedSquares(nums)

