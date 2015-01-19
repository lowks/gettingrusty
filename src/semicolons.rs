pub mod expr {
	pub fn expr() -> int {
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
                y
	}
}

#[test]                                                                                                                                                                                      

fn test_expr() {
    assert_eq!(9, expr::expr());
}
