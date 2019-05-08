use std::collections::HashMap;

// 8ms 2.3MB
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut row = HashMap::new();
    let mut col = HashMap::new();

    for (i, v) in matrix.iter().enumerate() {
        for (j, k) in v.iter().enumerate() {
            if k == &0 {
                row.insert(i, 1);
                col.insert(j, 1);
            }
        }
    }
    for i in 0..matrix.len() {
        match row.get(&i) {
            Some(_) => {
                matrix[i] = vec![0; matrix[i].len()];
                continue;
            }
            None => {
                let pre = &mut matrix[i];
                for (index, _) in &col {
                    pre[*index] = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);

        let mut matrix1 = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        set_zeroes(&mut matrix1);
        assert_eq!(matrix1, vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]);
    }
}
