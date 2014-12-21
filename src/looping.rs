fn main() {
	for x in range(0i, 10i) {
		println!( "{}", x);
	}

	println!("Iter function loop");

	let values = vec![1i, 2, 3, 4];
	for &x in values.iter() {
		println!("{}", x);
	}
}
