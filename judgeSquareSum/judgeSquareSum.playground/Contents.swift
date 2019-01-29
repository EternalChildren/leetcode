// Given a non-negative integer c, your task is to decide whether there're two integers a and b such that a2 + b2 = c.

let num = 4

// 264ms
func judgeSquareSum(_ c: Int) -> Bool {
    var hash = [Int: Int]()
    var i = 0
    var s = 0
    while s <= c {
        if s*2 == c || hash[c - s] != nil {
            return true
        } else {
            hash[s] = 1
            i += 1
            s = i*i
        }
    }
    return false
}

judgeSquareSum(num)
