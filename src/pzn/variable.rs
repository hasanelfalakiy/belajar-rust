#[cfg(test)]
mod variable_test {
    #[test]
    fn variable() {
        let name = "Andi Hasan Ashari"; // default immutable & hanya boleh assigment satu kali
        println!("Hello {}", name);

        let mut nama = "Joko"; // variable mutable
        println!("Hello {}", nama);

        nama = "Andi";
        println!("Hello {}", nama);
    }
}