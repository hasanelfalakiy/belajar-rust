#[cfg(test)]
mod augmented_assignment_test {
	
	#[test]
	fn augmented_assignment() {
	    
		let mut a = 10;
		println!("{}", a);
		
		a += 10;
		println!("{}", a);
		
		a -= 10;
		println!("{}", a);
	}
}