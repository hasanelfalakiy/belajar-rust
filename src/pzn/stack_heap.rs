#[cfg(test)]
mod stack_heap_tests {
    #[test]
    fn stack_heap() {

        function_a();
        function_b();
    }

    fn function_a() {
        let a = 10;
        let b = String::from("kurniawan");
        println!("{} {}", a, b);
    }

    fn function_b() {
        let a = 20;
        let b = String::from("eko");
        println!("{} {}", a, b);
    }
}