// Given a non-negative integer c, your task is to decide whether there're two integers a and b such that a2 + b2 = c.
import Foundation
let num = 4

// 264ms
func judgeSquareSum1(_ c: Int) -> Bool {
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

// 16ms
func judgeSquareSum2(_ c: Int) -> Bool {
    var i = 0
    while i*i <= c {
        let j = sqrt(Double(c - i*i))
        if j == floor(j) {
            return true
        }
        i += 1
    }
    return false
}

judgeSquareSum1(num)
judgeSquareSum2(num)
