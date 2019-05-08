// Given a 32-bit signed integer, reverse digits of an integer.

let nums = 120

// 12ms
func reverse(_ x: Int) -> Int {
    var x = x
    var res = 0
    while x != 0 {
        let k = x % 10
        res = res*10 + k
        if res > Int32.max || res < Int32.min {
            return 0
        }
        x /= 10
    }
    return res
}

reverse(nums)
