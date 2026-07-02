#[cfg(test)]
mod data_type_convertion_test {
	
	/*
	konversi tipe data number
	
	*/
	#[test]
	fn number_convertion() {
	    
		let a: i8 = 10;
		println!("{}", a);
		
		let b: i16 = a as i16;
		println!("{}", b);
		
		let c: i32 = b as i32;
		println!("{}", c);
		
		/* 
		contoh yg terkena integer overflow
		ini terjadi saat konversi dari tipe data number besar
		ke kecil, variable tipe data besar tidak dapat menampung nilainya
		*/
		let d: i64 = 12000000000;
		let e: i8 = d as i8;
		println!("{}", e);
	}
}