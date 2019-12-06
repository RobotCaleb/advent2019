extern crate day_1;

#[cfg(test)]
mod tests {
    #[test]
    fn fuel_12() {
        assert_eq!(day_1::sum_fuel(12), 2);
    }
    #[test]
    fn fuel_14() {
        assert_eq!(day_1::sum_fuel(14), 2);
    }
    #[test]
    fn fuel_1969() {
        assert_eq!(day_1::sum_fuel(1969), 966);
    }
    #[test]
    fn fuel_100756() {
        assert_eq!(day_1::sum_fuel(100756), 50346);
    }
}
