// You're given strings J representing the types of stones that are jewels, and S representing the stones you have.
// Each character in S is a type of stone you have.
// You want to know how many of the stones you have are also jewels.
// The letters in J are guaranteed distinct, and all characters in J and S are letters. Letters are case sensitive,
// so "a" is considered a different type of stone from "A".


var J = "aA"
var S = "aAAbbbb"

// 12ms
func numJewelsInStones(_ J: String, _ S: String) -> Int {
    var dic = [Character: Int]()
    var res = 0
    for (_, v) in S.enumerated() {
        if let count = dic[v] {
            dic[v] = count + 1
        } else {
            dic[v] = 1
        }
    }
    for (_, j) in J.enumerated() {
        if let count = dic[j] {
            res += count
        }
    }
    return res
}


numJewelsInStones(J, S)
