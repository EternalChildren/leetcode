// Implement strStr().

// Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
import Foundation
let str = "b"
let targe = "b"

//  24ms
func strStr(_ haystack: String, _ needle: String) -> Int {
    let lenH = haystack.count
    let lenN = needle.count
    guard lenN > 0 || (lenH > 0 && lenN > 0) else {
        return 0
    }
    guard lenH >= lenN else {
        return -1
    }
    let limit = lenH - lenN
    for i in 0...limit {
        let range = String.Index(encodedOffset: i)..<String.Index(encodedOffset: lenN + i)
        if haystack.substring(with: range) == needle {
            return i
        }
    }
    return -1
}


strStr(str, targe)
