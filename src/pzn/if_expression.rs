#[cfg(test)]

mod if_expression_test {

    #[test]
    fn if_expression() {
        let value = 9;

        if value >= 8 {
            // ekspresi perbandingan di if tidak wajib menggunakan kurung buka tutup (), jika menggunakan malah ada warning "remove these parentheses"
            println!("Good")
        } else if value >= 6 {
            println!("Not Bad")
        } else if value >= 3 {
            println!("Bad")
        } else {
            println!("Very Bad")
        }
    }
}
