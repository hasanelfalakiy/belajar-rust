fn main() {
    println!("Hello rust!");
	// ini komentar single line
	
	/*
	* ini komentar
	* baris multi 
	* atau banyak baris
	*/
}

// cara menjalankan fungsi ini tanpa perlu meneruskan ke main func
// terminal$: cargo test nama_fungsi_test -- --nocapture
#[test]
fn variable() {
    let nama_variable = "predifined value ";
	print!("{}", nama_variable);
}
