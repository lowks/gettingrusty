use std::os;

fn main() {
	let args = os::args();
	println!("The argument is {}", args[1]);
}
