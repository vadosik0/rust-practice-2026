#[allow(dead_code)]
pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(grades.len());

    for &grade in grades {
        let mut final_grade = grade;

        if grade >= 38 {
            let next_multiple = ((grade / 5) + 1) * 5;
            let diff = next_multiple - grade;

            if diff < 3 {
                final_grade = next_multiple;
            }
        }

        result.push(final_grade);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::grading_students;

    #[test]
    fn test_rounding_logic() {
        let input = [73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(&input), expected);
    }

    #[test]
    fn test_no_rounding_threshold() {
        let input = [37, 0, 100];
        let expected = vec![37, 0, 100];
        assert_eq!(grading_students(&input), expected);
    }

    #[test]
    fn test_execution_validity() {
        let dummy = [84];
        assert_eq!(grading_students(&dummy), vec![85]);
    }
}
