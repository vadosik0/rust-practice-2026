#[allow(dead_code)]
pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        return "NO".to_string();
    }

    let distance = x2 - x1;
    let speed_diff = v1 - v2;

    if distance % speed_diff == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::kangaroo;

    #[test]
    fn test_kangaroo_meet_case() {
        assert_eq!(kangaroo(2, 4, 6, 2), "YES");
    }

    #[test]
    fn test_kangaroo_no_meet_speed() {
        assert_eq!(kangaroo(0, 1, 4, 3), "NO");
    }

    #[test]
    fn test_kangaroo_same_speed() {
        assert_eq!(kangaroo(10, 3, 20, 3), "NO");
    }
}
