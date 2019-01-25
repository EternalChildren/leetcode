
var num = [Int]()

// 80ms
func removeDuplicates1(_ nums: inout [Int]) -> Int {
    guard nums.count > 0 else {
        return 0
    }
    var i = 0
    for j in 1..<nums.count {
        if nums[i] != nums[j] {
            i += 1
            nums[i] = nums[j]
        }
    }
    return i + 1
}

// 100ms
func removeDuplicates2(_ nums: inout [Int]) -> Int {
    var dic = [Int: Int]()
    for i in 0..<nums.count {
        dic[nums[i]] = i
    }
    nums = Array(dic.keys)
    nums.sort()
    return dic.keys.count
}


removeDuplicates1(&num)
removeDuplicates2(&num)

