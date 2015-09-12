pub mod looping {
    pub fn main() {
	for x in (0..10) {
	    println!( "{}", x);
	}

	println!("Iter function loop");

	let values = vec![1, 2, 3, 4];
	for &x in values.iter() {
	    println!("{}", x);
	}
    }
}
