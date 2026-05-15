pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut count = [0; 6];

    for &bird in arr {
        count[bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut result = 0;

    for i in 1..=5 {
        if count[i] > max_count {
            max_count = count[i];
            result = i as i32;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_sample_1() {
        let arr = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
        assert_eq!(migratory_birds(&arr), 3);
    }

    #[test]
    fn test_single_most_common() {
        let arr = vec![2, 2, 2, 1, 3];
        assert_eq!(migratory_birds(&arr), 2);
    }

    #[test]
    fn test_smallest_id_on_tie() {
        let arr = vec![1, 1, 2, 2, 3];
        assert_eq!(migratory_birds(&arr), 1);
    }
}