pub mod expr {
	pub fn expr() {
		let x = 5u;

		let y = {
			let j = 4u;
			let k = 5u;
			j + k
		};

		let z = { 
		    let m = 5u;
		    2 * m;
		};

		println!("x is {}", x);
		println!("y is {}", y);
		println!("z is {}", z);
	}
}
