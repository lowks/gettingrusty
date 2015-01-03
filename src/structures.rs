pub mod struct_example {

struct Pair {
	a: f64,
	b: f64,
}

	pub fn example () {
	
		println!("##### Struct Example #####");	
		let pair: Pair = Pair {a: 0.3, b: 0.4};
		println!("{} and {}", pair.a, pair.b);

	}

}
