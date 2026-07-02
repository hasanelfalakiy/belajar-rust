#[cfg(test)]
mod numeric_operator_test {
	
	#[test]
	fn numeric_operator() {
	    
		let a = 10;
		let b = 10;
		let c = a * b;
		println!("{}", c);
		
		let d = a / b;
		println!("{}", d);
		
		let e = a + b;
		println!("{}", e);
	}
}