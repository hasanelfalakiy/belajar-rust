#[cfg(test)]
mod data_copy_tests {

    #[test]
    fn data_copy() {

        let a = 10;
        let b = a;
        println!("{}, {}", a, b);
    }
}