
#[cfg(test)]
mod let_statement_test {

    /*
    If di Rust adalah expression, artinya bisa mengembalikan value/nilai dan bisa 
    digunakan dengan let statement untuk mengisi data di variable

    ini sangat berguna sehingga kita tidak perlu memasukkan 
    nilai ke variable terpisah dengan deklarasi variable nya
     */

    #[test]
    fn let_statement() {
        // contoh manual
        let value = 9;
        let result: &str;

        if value >= 8 {
            // ekspresi perbandingan di if tidak wajib menggunakan kurung buka tutup (), jika menggunakan malah ada warning "remove these parentheses"
            result = "Good"
        } else if value >= 6 {
            result = "Not Bad"
        } else if value >= 3 {
            result = "Bad"
        } else {
            result = "Very Bad"
        }

        println!("{}", result);

    }

    #[test]
    fn let_statement_simple() {
        let value = 9;
        let result = if value >= 8 {
            "Good"
        } else if value >= 6 {
            "Not Bad"
        } else if value >= 3 {
            "Bad"
        } else {
            "Very Bad"
        } ;

        println!("{}", result);
    }
}