fn main() {
    // no-op
}

fn fahrenheit_to_celsuius(fahrenheit: f32) -> f32 {
    return (fahrenheit - 32.0) * (5.0 / 9.0);
}

#[cfg(test)]
mod test_fahrenheit_to_celsuius {
    use super::*;

    #[test]
    fn should_convert_zero_fahrenheit_to_17_7778_celsius() {
        let actual = fahrenheit_to_celsuius(0.0);

        assert_eq!("-17.7778", format!("{:.4}", actual));
    }


    #[test]
    fn should_convert_fahrenheit_to_celsius() {
        assert_eq!("-12.2222", format!("{:.4}", fahrenheit_to_celsuius(10.0)));
        assert_eq!("37.7778", format!("{:.4}", fahrenheit_to_celsuius(100.0)));
        assert_eq!("38.0556", format!("{:.4}", fahrenheit_to_celsuius(100.5)));
        assert_eq!("93.3333", format!("{:.4}", fahrenheit_to_celsuius(200.0)));
    }
}
