#[cfg(test)]
mod constant_test {
	
	/* Constant
	- adalah variable immutable dg keyword const
	- bedanya const dg let, const tidak memiliki mutable
	- nilai const harus dideklarasikan saat program dibuat, bukan saat dijalankan,
	sebab itu nilai const tidak bisa hasil kalkulasi nilai lain yg belum jelas
	- membuat constant harus disebutkan tipe datanya secara eksplisit
	- nama const harus huruf besar semua dg pemisah _ (garis bawah))
	*/
	
	const MAXIMUM: i32 = 100;
	
	#[test]
	fn constant() {
	    
		const MINIMUM: i32 = 0;
		
		println!("{} {}", MINIMUM, MAXIMUM);
	}
}