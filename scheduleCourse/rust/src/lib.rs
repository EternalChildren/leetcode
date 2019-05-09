// 300ms 2.7MB
pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
    let mut course_cnt = vec![];
    let mut total_day = 0;
    courses.sort_by(|a, b| {
        if a[1] == b[1] {
            a[0].partial_cmp(&b[0]).unwrap()
        } else {
            a[1].partial_cmp(&b[1]).unwrap()
        }
    });
    println!("courses is {:?}", courses);
    let max_date = courses[courses.len() - 1][1];
    println!("max_date is {}", max_date);

    for course in courses {
        if total_day + course[0] <= course[1] {
            total_day += course[0];
            course_cnt.push(course[0]);
            course_cnt.sort();
        } else {
            match course_cnt.last() {
                Some(&last) => {
                    if last > course[0] {
                        total_day = total_day - last + course[0];
                        course_cnt.pop();
                        course_cnt.push(course[0]);
                        course_cnt.sort();
                    }
                }
                None => {}
            }
        }
    }

    course_cnt.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec = vec![
            vec![100, 200],
            vec![200, 1300],
            vec![1000, 1250],
            vec![2000, 3200],
        ];

        assert_eq!(schedule_course(vec), 3);
        let vec1 = vec![vec![5, 5], vec![4, 6], vec![2, 6]];

        assert_eq!(schedule_course(vec1), 2);

        let vec2 = vec![
            vec![7, 17],
            vec![3, 12],
            vec![10, 20],
            vec![9, 10],
            vec![5, 20],
            vec![10, 19],
            vec![4, 18],
        ];

        assert_eq!(schedule_course(vec2), 4);

        let vec3 = vec![
            vec![7, 16],
            vec![2, 3],
            vec![3, 12],
            vec![3, 14],
            vec![10, 19],
            vec![10, 16],
            vec![6, 8],
            vec![6, 11],
            vec![3, 13],
            vec![6, 16],
        ];
        assert_eq!(schedule_course(vec3), 4);
    }
}
