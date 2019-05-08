// Given an array A of non-negative integers, return an array consisting of all the even elements of A, followed by all the odd elements of A.

// You may return any answer array that satisfies this condition.

let nums = [3,1,2,4]

// 92 ms
func sortArrayByParity(_ A: [Int]) -> [Int] {
    var res = [Int]()
    for a in A {
        if a%2 == 0 {
            res.insert(a, at: 0)
        }else{
            res.append(a)
        }
    }
    return res
}

sortArrayByParity(nums)
