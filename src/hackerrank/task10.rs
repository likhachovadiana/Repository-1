pub fn breaking_records(scores: Vec<i32>) -> Vec<i32> {
    let mut max_score = scores[0];
    let mut min_score = scores[0];

    let mut max_breaks = 0;
    let mut min_breaks = 0;

    for &score in scores.iter().skip(1) {
        if score > max_score {
            max_score = score;
            max_breaks += 1;
        }

        if score < min_score {
            min_score = score;
            min_breaks += 1;
        }
    }

    vec![max_breaks, min_breaks]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records_example() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(scores), vec![2, 4]);
    }

    #[test]
    fn test_breaking_records_no_changes() {
        let scores = vec![5, 5, 5, 5];
        assert_eq!(breaking_records(scores), vec![0, 0]);
    }
}