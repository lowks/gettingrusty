pub fn test() {
    let a = 10;
    if a < 100 || a > 9 {
        println!("helloworld from conditionals");
    }

    if a > 11 && a > 12 {
	println!("This will never show!");
    } else { println!("This will show up on the console"); }
}

pub fn main() {
	test();
}
