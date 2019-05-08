// International Morse Code defines a standard encoding where each letter is mapped to a series of dots and dashes, as follows: "a" maps to ".-", "b" maps to "-...", "c" maps to "-.-.", and so on.

// For convenience, the full table for the 26 letters of the English alphabet is given below:

// Now, given a list of words, each word can be written as a concatenation of the Morse code of each letter. For example, "cba" can be written as "-.-..--...", (which is the concatenation "-.-." + "-..." + ".-"). We'll call such a concatenation, the transformation of a word.

// Return the number of different transformations among all words we have.

let words = ["gin", "zen", "gig", "msg"]

// 24 ms
func uniqueMorseRepresentations(_ words: [String]) -> Int {
    let code:[Character: String] = [
        "a":".-", "b":"-...", "c":"-.-.",
        "d":"-..", "e":".", "f":"..-.",
        "g":"--.", "h":"....", "i":"..",
        "j":".---", "k":"-.-", "l":".-..",
        "m":"--", "n":"-.", "o":"---",
        "p":".--.", "q":"--.-", "r":".-.",
        "s":"...", "t":"-", "u":"..-",
        "v":"...-", "w":".--", "x":"-..-",
        "y":"-.--", "z":"--.."
    ]
    var res = [String: Int]()
    for word in words {
        var key = ""
        for c in word {
            key += code[c]!
        }
        if res[key] == nil {
            res[key] = 1
        }
    }
    return res.keys.count
}


uniqueMorseRepresentations(words)
