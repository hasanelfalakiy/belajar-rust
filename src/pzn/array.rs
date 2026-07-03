#[cfg(test)]
mod array_test {
	
	/*
	Array adalah kumpulan data sama seperti tuple,
	tapi array harus sama semua tipe datanya
	untuk membuat array gunakan tanda [] kurung kotak
	*/
	#[test]
	fn array() {
	    // jika mau disebut secara eksplisit
		// let angka: [i32, 5] = [1, 2, 3, 4, 5];
		let angka = [1, 2, 3, 4, 5];
		
		println!("{:?}", angka);
	}
}