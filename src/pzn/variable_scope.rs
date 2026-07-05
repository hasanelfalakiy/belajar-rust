#[cfg(test)]
mod variable_scope_test {
	
	/* Variable Scope
	- variable scope mendefinisikan area dimana variable bisa digunakan
	- variable bisa digunakan didalam scope tempat variable didefinisikan, dan didalam inner scope (scope yg lebih dalam)
	- variable tidak bisa digunakan di outer scope (luar scope)
	*/
	#[test]
	fn variable_scope() {
	    
		let eko = "eko";
		
		{// inner scope
			println!("inner eko: {}", eko);
			let kurniawan = "kurniawan";
			println!("inner kurniawan: {}", kurniawan);
		}
		
		/* outer scope
		println!("outer scope: {}", kurniawan); // error
		*/
	}
}