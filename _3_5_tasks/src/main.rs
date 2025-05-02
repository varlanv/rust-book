fn main() {
    fahrenheit_to_celsuius(0.0);
    fibonacci(1);
}

fn fahrenheit_to_celsuius(fahrenheit: f32) -> f32 {
    return (fahrenheit - 32.0) * (5.0 / 9.0);
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut res: u64 = 1;
    let mut prev: u64 = 0;
    for _i in 0..(n - 1) {
        let current = res;
        res = res + prev;
        prev = current;
    }
    return res;
}

#[cfg(test)]
mod test_fahrenheit_to_celsuius {
    use super::*;

    #[test]
    fn should_convert_zero_fahrenheit_to_17_7778_celsius() {
        let actual = fahrenheit_to_celsuius(0.0);

        assert_eq!(format!("{:.4}", actual), "-17.7778");
    }


    #[test]
    fn should_convert_fahrenheit_to_celsius() {
        assert_eq!(format!("{:.4}", fahrenheit_to_celsuius(10.0)), "-12.2222");
        assert_eq!(format!("{:.4}", fahrenheit_to_celsuius(100.0)), "37.7778");
        assert_eq!(format!("{:.4}", fahrenheit_to_celsuius(100.5)), "38.0556");
        assert_eq!(format!("{:.4}", fahrenheit_to_celsuius(200.0)), "93.3333");
    }
}

#[cfg(test)]
mod test_fibonacci {
    use super::*;

    #[test]
    fn should_calculate_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
        assert_eq!(fibonacci(30), 832040);
    }
}
