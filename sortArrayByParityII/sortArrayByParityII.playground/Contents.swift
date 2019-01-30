// Given an array A of non-negative integers, half of the integers in A are odd, and half of the integers are even.

// Sort the array so that whenever A[i] is odd, i is odd; and whenever A[i] is even, i is even.

// You may return any answer array that satisfies this condition.

let nums = [4,2,5,7]

// 236ms
func sortArrayByParityII(_ A: [Int]) -> [Int] {
    var odd = [(Int, Int)]()
    var even = [(Int, Int)]()
    var res = A
    for i in 0..<res.count {
        if i%2 == 0 && res[i]%2 == 0 {
            continue
        }
        if i%2 != 0 && res[i]%2 != 0 {
            continue
        }
        if i%2 == 0 {
            odd.append((i, res[i]))
        } else {
            even.append((i, res[i]))
        }
    }
    for k in 0..<odd.count {
        res[odd[k].0] = even[k].1
        res[even[k].0] = odd[k].1
    }
    return res
}

sortArrayByParityII(nums)
