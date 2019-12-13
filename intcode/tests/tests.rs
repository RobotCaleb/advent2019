extern crate intcode;

#[cfg(test)]
mod tests {
    fn test(input: Vec<i32>, truth: Vec<i32>) {
        let mut prog = intcode::IntCode::new();
        prog.load(&input);
        prog.run();
        assert_eq!(prog.state, truth);
    }

    #[test]
    fn test_computer_add_1() {
        test(
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
    }

    #[test]
    fn test_computer_add_2() {
        test(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_computer_mul_3() {
        test(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_computer_mul_4() {
        test(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_computer_add_5() {
        test(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }

    // #[test]
    // // input value of 1 for test to pass
    // fn test_computer_input_6() {
    //     test(vec![3, 3, 99, 0], vec![3, 3, 99, 1]);
    // }

    // #[test]
    // // input value of 1 for test to pass
    // fn test_computer_input_output_6() {
    //     test(vec![3, 0, 4, 0, 99], vec![1, 0, 4, 0, 99]);
    // }
}
