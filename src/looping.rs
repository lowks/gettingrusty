pub mod looping {
fn print_stuff() {
	for x in 0..10 {
	    println!( "{}", x);
	}

	println!("Iter function loop");

	let values = vec![1, 2, 3, 4];
	for &x in values.iter() {
	    println!("{}", x);
	}
}
	pub fn main() {
	   print_stuff();
	}
}


