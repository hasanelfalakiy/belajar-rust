#[cfg(test)]
mod comparison_operators_test {
    #[test]
    fn comparison_operators() {
        let a = 21;
        let b = 20;

        let result: bool = a > b;
        println!("{}", result);
    }
}