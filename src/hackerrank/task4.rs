pub fn mini_max_sum(arr: &[i64]) -> (i64, i64) {
    let total: i64 = arr.iter().sum();

    let min = arr.iter().map(|&x| total - x).min().unwrap();
    let max = arr.iter().map(|&x| total - x).max().unwrap();

    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mini_max_sum() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(mini_max_sum(&arr), (10, 14));
    }
}