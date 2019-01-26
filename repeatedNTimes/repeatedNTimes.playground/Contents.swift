// In a array A of size 2N, there are N+1 unique elements, and exactly one of these elements is repeated N times.

// Return the element repeated N times.

let nums = [5,1,5,2,5,3,5,4]

// 324ms
func repeatedNTimes1(_ A: [Int]) -> Int {
    var hash = [Int: Int]()
    let n = A.count / 2
    for a in A {
        if let count = hash[a] {
            hash[a] = count + 1
        } else {
            hash[a] = 1
        }
    }
    for (key,value) in hash {
        if value == n {
            return key
        }
    }
    return 0
}

// 根据条件可以判定N占了总数组的一半，并且其他数据不会重复，因为可以得出条件只要两者索引不同并且数值相同即答案所需结果
// 276ms
func repeatedNTimes2(_ A: [Int]) -> Int {
    var i = 0
    var j = 0
    repeat {
        i = Int.random(in: 0..<A.count)
        j = Int.random(in: 0..<A.count)
    } while i == j || A[i] != A[j]
    return A[i]
}

repeatedNTimes1(nums)
repeatedNTimes2(nums)
