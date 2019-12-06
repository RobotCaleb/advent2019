extern crate day_2;

#[cfg(test)]
mod tests {
    fn test(input: Vec<i32>, truth: Vec<i32>) {
        let mut prog = day_2::IntCode::new();
        prog.load(&input);
        prog.run();
        assert_eq!(prog.state, truth);
    }

    #[test]
    fn test_1() {
        test(
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
    }

    #[test]
    fn test_2() {
        test(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_3() {
        test(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_4() {
        test(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_5() {
        test(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }
}
