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
        test_output(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 0, Some(1));
        test_output(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 1, Some(8));
    }

    #[test]
    fn test_less_than_1() {
        test_output(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 1, Some(7));
        test_output(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 0, Some(9));
    }

    #[test]
    fn test_equal_2() {
        test_output(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 1, Some(8));
        test_output(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 0, Some(7));
    }

    #[test]
    fn test_less_than_2() {
        test_output(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 1, Some(7));
        test_output(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 0, Some(9));
    }

    #[test]
    fn test_jump_1() {
        test_output(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            0,
            Some(0),
        );
        test_output(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            1,
            Some(12),
        );
    }

    #[test]
    fn test_jump_2() {
        test_output(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            0,
            Some(0),
        );
        test_output(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            1,
            Some(12),
        );
    }

    #[test]
    fn test_larger() {
        test_output(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            999,
            Some(7),
        );
        test_output(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            1000,
            Some(8),
        );
        test_output(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            1001,
            Some(9),
        );
    }
}
