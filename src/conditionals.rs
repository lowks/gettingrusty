pub mod conditional {
	pub fn tester(a: i32) -> i32 {
	    if a < 100 || a > 9 {
		println!("helloworld from conditionals");
	    }

	    if a > 11 && a > 12 {
		return 10;
	    } else { return 0; }

	}
        pub fn main() {
            let x = 5;
            let y = if x == 5 {10} else {15};
            println!("y is {}", y);
	    assert_eq!(tester(10), 0);
	}
}


#[test]

fn test_conditionals(){
    assert_eq!(conditional::tester(10), 0);
    assert_eq!(conditional::tester(13), 10);
}
