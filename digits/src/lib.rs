pub mod digits {
    pub fn get_digits(input: usize) -> Vec<usize> {
        let mut ds = vec![];
        let mut i = input;
        while i > 0 {
            ds.push(i % 10);
            i = i / 10;
        }

        ds.reverse();
        ds
    }
}
