#![feature(old_io, old_path, os, old_fs)]
use std::old_path::{Path};
use std::old_io::{fs};
use std::old_io;

fn main() {
	let p = Path::new("tmp");
	fs::mkdir(&p, old_io::USER_RWX);
	fs::rmdir(&p);
}

