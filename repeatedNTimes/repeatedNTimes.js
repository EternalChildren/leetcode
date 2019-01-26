// In a array A of size 2N, there are N+1 unique elements, and exactly one of these elements is repeated N times.

// Return the element repeated N times.

const nums = [5, 1, 5, 2, 5, 3, 5, 4]

// 104ms
const repeatedNTimes1 = (A) => {
  let hash = new Map()
  const n = A.length / 2
  for (let a of A) {
    const count = hash.get(a)
    count ? hash.set(a, count + 1) : hash.set(a, 1)
  }
  for (let item of hash.entries()) {
    if (item[1] === n) {
      return item[0]
    }
  }
}

// 根据条件可以判定N占了总数组的一半，并且其他数据不会重复，因为可以得出条件只要两者索引不同并且数值相同即答案所需结果
// 80ms
const repeatedNTimes2 = (A) => {
  let i = 0
  let j = 0

  do {
    i = Math.floor(Math.random() * A.length)
    j = Math.floor(Math.random() * A.length)
  } while (i === j || A[i] != A[j])
  return A[i]
}

// 76ms 最初始的for循环 速度快于 for of 等语法糖
const repeatedNTimes3 = (A) => {
  let map = {}
  for (let i = 0; i < A.length; i++) {
    if (map[A[i]]) {
      return A[i]
    } else {
      map[A[i]] = 1
    }
  }
}

const res1 = repeatedNTimes1(nums)
const res2 = repeatedNTimes2(nums)
const res3 = repeatedNTimes3(nums)
