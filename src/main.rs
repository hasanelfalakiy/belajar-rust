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
	println!("{}", nama_variable);
	
	let mut message_number = 1;
	let message1 = "hello";
	println!("message number {}: {}", message_number, message1);
	
	message_number = 2; // boleh diubah jika variable dideklarasikan sebagai mutable dengan keyword mut
	let message2 = "world";
	println!("message number {}: {}", message_number, message2);
	
	// argument parameter macro
	message_number = 3;
	let message3: i8 = 24;
	/* terlihat seperti terbalik tapi lihat argumen indeks yang 
	*  dimasukkan ke parameter macro di set ke 1 dulu baru ke 0
	*/
	println!("message number {1}: {0}", message3, message_number);
}

