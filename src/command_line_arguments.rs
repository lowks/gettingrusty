use std::os;

fn main() {
	let args = os::args();
	let first_args = &args[1].to_string();
	match &**first_args {
		"test" => println!("Good!"),
		_ => println!("No test ?!"),
	}
}
