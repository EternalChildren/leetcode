// The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1. That is,

// Given N, calculate F(N).

let num = 3

// 32ms
func fib(_ N: Int) -> Int {
    if N == 0 {
        return 0
    }
    if N == 1 {
        return 1
    }
    return fib(N - 1) + fib(N - 2)
}

fib(num)

