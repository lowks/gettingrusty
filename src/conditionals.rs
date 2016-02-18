pub mod conditional {
	pub fn tester() -> i32 {
	    let a = 10;
	    if a < 100 || a > 9 {
		println!("helloworld from conditionals");
	    }

	    if a > 11 && a > 12 {
		return 10;
	    } else { return 0; }
	}
	
	pub fn main() {
	   assert_eq!(tester(), 0);
	}
}


#[test]

fn test_conditionals(){
    assert_eq!(conditional::tester(), 0);
}
