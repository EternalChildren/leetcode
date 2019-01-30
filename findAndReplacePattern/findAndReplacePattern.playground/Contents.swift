// You have a list of words and a pattern, and you want to know which words in words matches the pattern.

// A word matches the pattern if there exists a permutation of letters p so that after replacing every letter x in the pattern with p(x), we get the desired word.

// (Recall that a permutation of letters is a bijection from letters to letters: every letter maps to another letter, and no two letters map to the same letter.)

// Return a list of the words in words that match the given pattern.

// You may return the answer in any order.
import Foundation


let words = ["abc","deq","mee","aqq","dkd","ccc"]
let pattern = "abb"

// 20ms
func findAndReplacePattern(_ words: [String], _ pattern: String) -> [String] {
    var res = [String]()
    for w in words {
        if match(w, pattern) {
            res.append(w)
        }
    }
    return res
}

func match(_ word: String, _ pattern: String) -> Bool {
    var hash = [Character: Character]()
    
    for i in 0..<word.count {
        let index = String.Index(encodedOffset: i)
        let w = word[index]
        let p = pattern[index]
        if !hash.keys.contains(w)  {
            hash[w] = p
        }
        if hash[w] != p {
            return false
        }
    }
    var seen:[Bool] = Array(repeating: false, count: 26)
    let unicode_a = Character("a").unicodeScalars[Character("a").unicodeScalars.startIndex]
    for v in hash.values {
        if seen[Int(v.unicodeScalars[v.unicodeScalars.startIndex].value) - Int(unicode_a.value)] {
            return false
        }
        seen[Int(v.unicodeScalars[v.unicodeScalars.startIndex].value) - Int(unicode_a.value)] = true
    }
    return true
}

findAndReplacePattern(words, pattern)

