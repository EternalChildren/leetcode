// 0ms 2.1MB
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.len() < 1 || matrix[0].len() < 1 {
        return false;
    }
    for v in matrix.iter() {
        if target >= v[0] && target <= v[v.len() - 1] {
            for i in v {
                if i == &target {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let martix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]];

        assert_eq!(search_matrix(martix.clone(), 3), true);

        assert_eq!(search_matrix(martix, 13), false);

        assert_eq!(search_matrix(vec![vec![]], 1), false);
    }
}
