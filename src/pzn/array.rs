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
		// let angka: [i32; 5] = [1, 2, 3, 4, 5];
		let angka = [1, 2, 3, 4, 5];
		
		println!("{:?}", angka);
		
		// cara mengakses array dengan [indeks], indeks dimulai dari 0
		let a = angka[0];
		let b = angka[1];
		println!("{} {}", a, b);
		
		/* 
		sama seperti tuple, jika variable array nya dibuat immutable
		maka isi data array tidak dapat diubah, jika ingin bisa diubah maka harus dibuat mutable
		, untuk mengubah isi data array dengan [indeks] = nilai baru
		*/
		let mut num = [1, 2, 3, 4, 5];
		
		num[0] = 10;
		num[1] = 20;
		println!("{:?}", num);
		
		/*
		Panjang Array
		yg membedakan dengan tuple, di array kita bisa mendapatkan panjang array
		dengan function bawaan array len() 
		*/
		let p = num.len();
		println!("{}", p);
		
		/*
		Dimensional Array
		saat membuat array, kita bisa menggunakan tipe data apapun
		termasuk array itu sendiri (array didalam array)
		*/
		let matrix: [[i32; 2]; 2] = [
			[1, 2],
			[3, 4]
		];
		
		println!("{:?}", matrix);
		println!("{}", matrix[0][0]);
		println!("{}", matrix[0][1]);
		println!("{}", matrix[1][0]);
		println!("{}", matrix[1][1]);
	}
}