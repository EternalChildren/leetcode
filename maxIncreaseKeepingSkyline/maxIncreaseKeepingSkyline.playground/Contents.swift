// In a 2 dimensional array grid, each value grid[i][j] represents the height of a building located there. We are allowed to increase the height of any number of buildings, by any amount (the amounts can be different for different buildings). Height 0 is considered to be a building as well.

// At the end, the "skyline" when viewed from all four directions of the grid, i.e. top, bottom, left, and right, must be the same as the skyline of the original grid. A city's skyline is the outer contour of the rectangles formed by all the buildings when viewed from a distance. See the following example.

// What is the maximum total sum that the height of the buildings can be increased?

let nums = [[3,0,8,4],[2,4,5,7],[9,2,6,3],[0,3,1,0]]

// 56ms
func maxIncreaseKeepingSkyline(_ grid: [[Int]]) -> Int {
    var tb = [Int]()
    var lr = [Int]()
    for i in grid {
        lr.append(i.max()!)
        if tb.count > 0 {
            for index in 0..<i.count {
                if i[index] > tb[index] {
                    tb[index] = i[index]
                }
            }
        }else{
            tb = i
        }
    }
    var res = 0
    for i in 0..<grid.count {
        for j in 0..<grid[i].count {
            let max = lr[i] <= tb[j] ? lr[i] : tb[j]
            res += max - grid[i][j]
        }
    }
    return res
}


maxIncreaseKeepingSkyline(nums)
