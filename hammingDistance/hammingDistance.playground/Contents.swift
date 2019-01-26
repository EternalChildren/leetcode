// The Hamming distance between two integers is the number of positions at which the corresponding bits are different.

// Given two integers x and y, calculate the Hamming distance.

// Note: 0 ≤ x, y < 231.

func pad(string : String, toSize: Int) -> String {
    var padded = string
    for _ in 0..<(toSize - string.characters.count) {
        padded = "0" + padded
    }
    return padded
}

let x = 1
let y = 4

// 8ms 事实证明涉及二进制 还是位运算最为快速简洁
func hammingDistance1(_ x: Int, _ y: Int) -> Int {
    return (x ^ y).nonzeroBitCount
}

// 12ms
func hammingDistance2(_ x: Int, _ y: Int) -> Int{
    guard x >= 0 && y < 4294967296 else {
        return 0
    }
    let a = pad(string: String(x, radix: 2, uppercase: false), toSize: 32)
    let b = pad(string: String(y, radix: 2, uppercase: false), toSize: 32)
    var res = 0
    for i in 0..<32 {
        if a[a.index(a.startIndex, offsetBy: i)] != b[b.index(b.startIndex, offsetBy: i)] {
            res += 1
        }
    }
    return res
}

hammingDistance1(x, y)
hammingDistance2(x, y)

