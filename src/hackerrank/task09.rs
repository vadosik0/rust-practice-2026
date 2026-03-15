#[allow(dead_code)]
pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut frequency = [0; 6];

    for i in 0..arr.len() {
        let id = arr[i] as usize;
        frequency[id] += 1;
    }

    let mut best_id = 1;
    let mut best_count = frequency[1];

    for i in 2..6 {
        if frequency[i] > best_count {
            best_count = frequency[i];
            best_id = i;
        }
    }

    best_id as i32
}

#[cfg(test)]
mod tests {
    use super::migratory_birds;

    #[test]
    fn test_most_frequent() {
        let data = [1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&data), 4);
    }

    #[test]
    fn test_tie_smallest_id() {
        let data = [1, 2, 2, 1, 3];
        assert_eq!(migratory_birds(&data), 1);
    }
}