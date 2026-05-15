pub fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut lcm_value = a[0];
    for &num in &a {
        lcm_value = lcm(lcm_value, num);
    }

    let mut gcd_value = b[0];
    for &num in &b {
        gcd_value = gcd(gcd_value, num);
    }

    let mut count = 0;
    let mut i = lcm_value;

    while i <= gcd_value {
        if gcd_value % i == 0 {
            count += 1;
        }
        i += lcm_value;
    }

    count
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x_example() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(a, b), 3);
    }

    #[test]
    fn test_get_total_x_second() {
        let a = vec![3, 4];
        let b = vec![24, 48];
        assert_eq!(get_total_x(a, b), 2);
    }
}