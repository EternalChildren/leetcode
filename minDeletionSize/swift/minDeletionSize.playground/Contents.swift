// We are given an array A of N lowercase letter strings, all of the same length.

// Now, we may choose any set of deletion indices, and for each string, we delete all the characters in those indices.

//  For example, if we have an array A = ["abcdef","uvwxyz"] and deletion indices {0, 2, 3}, then the final array after deletions is ["bef", "vyz"], and the remaining columns of A are ["b","v"], ["e","y"], and ["f","z"].  (Formally, the c-th column is [A[0][c], A[1][c], ..., A[A.length-1][c]].)

// Suppose we chose a set of deletion indices D such that after deletions, each remaining column in A is in non-decreasing sorted order.

// Return the minimum possible value of D.length.

let nums = ["cba","daf","ghi"]

// 1956ms 此问题不适合使用swift
func minDeletionSize(_ A: [String]) -> Int {
    var res = 0
    var hash = [Int: String]()
    for s in A {
        for (i,v) in s.enumerated() {
            if let str = hash[i] {
                hash[i] = str + "\(v)"
            }else{
                hash[i] = "\(v)"
            }
        }
    }
    for v in hash.values {
        for i in 0..<v.count-1 {
            if v[v.index(v.startIndex, offsetBy: i)] > v[v.index(v.startIndex, offsetBy: i + 1)] {
                res += 1
                break
            }
        }
    }
    return res
}

minDeletionSize(nums)
