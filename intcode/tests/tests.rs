extern crate intcode;

#[cfg(test)]
mod tests {
    fn test_against_truth(prog: Vec<i32>, truth: Vec<i32>, input: Option<i32>) {
        let mut comp = intcode::IntCode::new();
        comp.load(&prog);
        match input {
            Some(i) => comp.set_input(i),
            None => {}
        }
        while !comp.get_halted() {
            comp.step();
        }
        assert_eq!(comp.get_state(), truth);
    }

    fn test_output(prog: Vec<i32>, output: i32, input: Option<i32>) {
        let mut comp = intcode::IntCode::new();
        comp.load(&prog);
        match input {
            Some(i) => comp.set_input(i),
            None => {}
        }
        while !comp.get_halted() {
            comp.step();
        }
        assert_eq!(comp.get_output(), output);
    }

    #[test]
    fn test_computer_add_1() {
        test_against_truth(
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            None,
        );
    }

    #[test]
    fn test_computer_add_2() {
        test_against_truth(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99], None);
    }

    #[test]
    fn test_computer_mul_3() {
        test_against_truth(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99], None);
    }

    #[test]
    fn test_computer_mul_4() {
        test_against_truth(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801], None);
    }

    #[test]
    fn test_computer_add_5() {
        test_against_truth(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            None,
        );
    }

    #[test]
    fn test_param_modes() {
        test_against_truth(vec![1002, 4, 3, 4, 33], vec![1002, 4, 3, 4, 99], None);
        test_against_truth(vec![2, 4, 3, 5, 99, 0], vec![2, 4, 3, 5, 99, 5 * 99], None);
    }

    #[test]
    fn test_computer_input_6() {
        test_against_truth(vec![3, 3, 99, 0], vec![3, 3, 99, 1], Some(1));
    }

    #[test]
    fn test_computer_input_output_6() {
        test_against_truth(vec![3, 0, 4, 0, 99], vec![1, 0, 4, 0, 99], Some(1));
    }

    #[test]
    fn test_equal_1() {
        test_output(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 1, Some(1));
    }
}
