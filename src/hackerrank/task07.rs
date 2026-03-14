#[allow(dead_code)]
fn greatest_common_divisor(mut m: i32, mut n: i32) -> i32 {
    while n != 0 {
        let tmp = n;
        n = m % n;
        m = tmp;
    }
    m
}

#[allow(dead_code)]
fn least_common_multiple(x: i32, y: i32) -> i32 {
    if x == 0 || y == 0 {
        0
    } else {
        (x * y) / greatest_common_divisor(x, y)
    }
}

#[allow(dead_code)]
pub fn count_numbers_between_sets(set_a: &[i32], set_b: &[i32]) -> i32 {
    if set_a.is_empty() || set_b.is_empty() {
        return 0;
    }

    let lcm_a = set_a.iter().copied().fold(1, |acc, val| least_common_multiple(acc, val));

    let gcd_b = set_b.iter().copied().fold(set_b[0], |acc, val| greatest_common_divisor(acc, val));

    if lcm_a == 0 || lcm_a > gcd_b {
        return 0;
    }

    (1..=gcd_b / lcm_a)
        .filter(|&k| (lcm_a * k) % gcd_b == 0 || gcd_b % (lcm_a * k) == 0)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let a = [2, 4];
        let b = [16, 32, 96];
        assert_eq!(count_numbers_between_sets(&a, &b), 3);
    }

    #[test]
    fn test_case_2() {
        let a = [3, 4];
        let b = [24, 48];
        assert_eq!(count_numbers_between_sets(&a, &b), 2);
    }
}
