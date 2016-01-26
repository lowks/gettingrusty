fn main() {
	fn going_panic(s: &str) -> ! {
		println!("{}" ,s);
		panic!();
	}
	going_panic("Panic!");
}
