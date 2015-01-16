pub mod expr {
	pub fn expr() {
		let x = 5;

		let y = {
			let j = 4;
			let k = 5;
			j + k
		};

		let z = { 
		    let m = 5;
		    2 * m;
		};

		println!("x is {}", x);
		println!("y is {}", y);
		println!("z is {:?}", z);
	}
}
