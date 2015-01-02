use std::io;
use std::io::fs;

fn main() {
	let p = Path::new("tmp");
	fs::mkdir(&p, io::UserRWX);
	fs::rmdir(&p);
}

