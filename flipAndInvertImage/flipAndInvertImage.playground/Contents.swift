// Given a binary matrix A, we want to flip the image horizontally, then invert it, and return the resulting image.

// To flip an image horizontally means that each row of the image is reversed.  For example, flipping [1, 1, 0] horizontally results in [0, 1, 1].

// To invert an image means that each 0 is replaced by 1, and each 1 is replaced by 0. For example, inverting [0, 1, 1] results in [1, 0, 0].

let nums = [[1,1,0],[1,0,1],[0,0,0]]

// 40ms swift提供的原生方法能完美解决此问题
func flipAndInvertImage(_ A: [[Int]]) -> [[Int]] {
    var res = A
    for i in 0..<res.count {
        res[i].reverse()
        res[i] = res[i].map({$0 == 1 ? 0 : 1})
    }
    return res
}

flipAndInvertImage(nums)




