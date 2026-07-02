#[cfg(test)]

mod shadowing_test {

    #[test]
    fn shadowing() {
        let name = "Andi Hasan Ashari";
        println!("Hello {}", name);
        // shadowing variable name atas tertutupi variable name yg bawah
        let name = 10;
        println!("Hello {}", name);
    }
}