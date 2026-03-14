#[allow(dead_code)]
pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0];
    }

    let mut max_score = scores[0];
    let mut min_score = scores[0];
    let mut max_breaks = 0;
    let mut min_breaks = 0;

    for &s in &scores[1..] {
        if s > max_score {
            max_score = s;
            max_breaks += 1;
        }
        if s < min_score {
            min_score = s;
            min_breaks += 1;
        }
    }

    vec![max_breaks, min_breaks]
}

#[cfg(test)]
mod tests {
    use super::breaking_records;

    #[test]
    fn test_record_breaking() {
        let season_scores = [10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(&season_scores), vec![2, 4]);
    }

    #[test]
    fn test_short_season() {
        let season_scores = [3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        assert_eq!(breaking_records(&season_scores), vec![4, 0]);
    }
}
