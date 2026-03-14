#[allow(dead_code)]
fn calculate_fruits_in_range(s: i32, t: i32, tree_pos: i32, fruits: &[i32]) -> usize {
    let mut count = 0;

    for &distance in fruits {
        let landing = tree_pos + distance;

        if landing >= s && landing <= t {
            count += 1;
        }
    }

    count
}

#[allow(dead_code)]
pub fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_hits = calculate_fruits_in_range(s, t, a, apples);
    let orange_hits = calculate_fruits_in_range(s, t, b, oranges);

    println!("{}", apple_hits);
    println!("{}", orange_hits);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fruits_in_range() {
        let house_start = 7;
        let house_end = 11;

        let apple_tree = 5;
        let apples = [-2, 2, 1];
        assert_eq!(calculate_fruits_in_range(house_start, house_end, apple_tree, &apples), 1);

        let orange_tree = 15;
        let oranges = [5, -6];
        assert_eq!(calculate_fruits_in_range(house_start, house_end, orange_tree, &oranges), 1);
    }

    #[test]
    fn test_execution() {
        count_apples_and_oranges(7, 11, 5, 15, &[-2, 2, 1], &[5, -6]);
    }
}
