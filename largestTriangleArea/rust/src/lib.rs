// 0ms 2MB
pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let mut res: f64 = -1.0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            for k in j + 1..points.len() {
                let area = (points[i][0] * points[j][1]
                    + points[j][0] * points[k][1]
                    + points[k][0] * points[i][1]
                    - points[i][0] * points[k][1]
                    - points[j][0] * points[i][1]
                    - points[k][0] * points[j][1]) as f64
                    / 2.0;
                res = res.max(area.abs());
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let points = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]];

        assert_eq!(largest_triangle_area(points), 2.0);
    }
}
