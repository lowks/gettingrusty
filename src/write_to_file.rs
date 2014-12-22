fn main() {
	use std::io::File;
	let path = Path::new("foo.txt");
	let mut file = File::create(&path);
	file.write(b"Merry Christmas hoho!");
}
