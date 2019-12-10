pub mod password {
    pub fn brute_1(start: usize, end: usize) -> usize {
        let mut possible = vec![];
        for i in start..end + 1 {
            if contains_double(i) && does_not_decrease(i) {
                possible.push(i);
            }
        }
        possible.len()
    }

    fn get_digits(input: usize) -> Vec<usize> {
        let mut ds = vec![];
        let mut i = input;
        while i > 0 {
            ds.push(i % 10);
            i = i / 10;
        }

        ds.reverse();
        ds
    }

    fn contains_double(input: usize) -> bool {
        let digits = get_digits(input);
        let mut first = true;
        let mut last = digits[0];
        for d in &digits {
            if first {
                first = false;
                continue;
            }
            if *d == last {
                return true;
            }
            last = *d;
        }
        false
    }

    fn does_not_decrease(input: usize) -> bool {
        let digits = get_digits(input);
        let mut first = true;
        let mut last = digits[0];
        for d in &digits {
            if first {
                first = false;
                continue;
            }
            if *d >= last {
                last = *d;
                continue;
            } else {
                return false;
            }
        }
        true
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn get_digits() {
            let digits = super::get_digits(12345);
            assert_eq!(digits, vec![1, 2, 3, 4, 5]);
        }
        #[test]
        fn contains_double() {
            assert_eq!(super::contains_double(112), true);
            assert_eq!(super::contains_double(123), false);
            assert_eq!(super::contains_double(1234556), true);
            assert_eq!(super::contains_double(1234566), true);
        }
        #[test]
        fn does_not_decrease() {
            assert_eq!(super::does_not_decrease(112), true);
            assert_eq!(super::does_not_decrease(123), true);
            assert_eq!(super::does_not_decrease(1234556), true);
            assert_eq!(super::does_not_decrease(1324566), false);
            assert_eq!(super::does_not_decrease(798), false);
        }
        #[test]
        fn day_4_1() {
            assert_eq!(
                super::contains_double(111111) && super::does_not_decrease(111111),
                true
            );
        }
        #[test]
        fn day_4_2() {
            assert_eq!(
                super::contains_double(223450) && super::does_not_decrease(223450),
                false
            );
        }
        #[test]
        fn day_4_3() {
            assert_eq!(
                super::contains_double(123789) && super::does_not_decrease(123789),
                false
            );
        }
    }
}
