// Given a string S that only contains "I" (increase) or "D" (decrease), let N = S.length.

// Return any permutation A of [0, 1, ..., N] such that for all i = 0, ..., N-1:
    
//    If S[i] == "I", then A[i] < A[i+1]
//    If S[i] == "D", then A[i] > A[i+1]


let str = "IDID"


// 双指针I正序添加，D逆序添加
// unicodeScalars:字符串的值表示为Unicode标量值的集合
// 64ms
func diStringMatch(_ S: String) -> [Int] {
    var res = [Int]()
    var i = 0
    var j = S.count
    for s in S.unicodeScalars {
        switch s {
        case "D":
            res.append(j)
            j -= 1
        default:
            res.append(i)
            i += 1
        }
    }
    res.append(i)
    return res
}

diStringMatch(str)
