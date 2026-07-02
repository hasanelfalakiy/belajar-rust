#[cfg(test)]

mod data_type_test {

    /*
    sebenarnya compiler bisa mendeteksi jenis tipe data pada variable yang tidak di tulis tipe datanya secara eksplisit
    tapi jika ingin menuliskannya eksplisit tetap diperbolehkan
     */
    #[test]
    fn explicit_variable() {
        let age: i32 = 20;
        println!("{}", age);
    }

    /*
    tipe data di rust dibagi 2 = Scalar & Compound type
    1 Scalar = tipe data yang hanya merepresentasikan satu value (nilai tunggal), contoh = integer, float, boolean, char
    2 Compound = tipe data yang merepresentasikan lebih dari satu value dalam satu type, contoh = tuple, dan array
     */
    #[test]
    fn data_type() {
		
		let a: i32 = 10;
		println!("{}", a);
		
		let b: f64 = 12.5;
		println!("{}", b);
    }
	
	
}