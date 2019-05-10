use std::collections::binary_heap::BinaryHeap;

// 8ms 2.5MB
pub fn kth_smallest_binary_heap(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let k = k as usize;
    let mut heap = BinaryHeap::with_capacity(k);

    for i in matrix {
        for j in i {
            if heap.len() < k {
                heap.push(j);
            } else if *heap.peek().unwrap() > j {
                heap.pop();
                heap.push(j);
            }
        }
    }
    heap.pop().unwrap()
}

// 12ms 2.6MB
pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut res: Vec<i32> = vec![];
    for i in matrix {
        for j in i {
            res.push(j);
        }
    }

    res.sort();

    res[(k - 1) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
        assert_eq!(kth_smallest(matrix, 8), 13);

        let matrix1 = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
        assert_eq!(kth_smallest_binary_heap(matrix1, 8), 13);
    }
}
