// Given two integers A and B, return any string S such that:

// S has length A + B and contains exactly A 'a' letters, and exactly B 'b' letters;
// The substring 'aaa' does not occur in S;
// The substring 'bbb' does not occur in S.


let A = 4
let B = 8
// 20ms
func strWithout3a3b(_ A: Int, _ B: Int) -> String {
    var str = ""
    var A = A
    var B = B
    
    while A > 0 || B > 0 {
        var writeA = false
        let count = str.count
        if str.count >= 2 && str[str.index(before: str.endIndex)] == str[str.index(str.endIndex, offsetBy: -2)] {
            if str[str.index(before: str.endIndex)] == "b" {
                writeA = true
            }
        }else{
            if A > B {
                writeA = true
            }
        }
        
        if writeA {
            A -= 1
            str.append("a")
        } else {
            B -= 1
            str.append("b")
        }
    }
    return str
}

strWithout3a3b(A, B)

