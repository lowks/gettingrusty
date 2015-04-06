pub mod guard {
    	pub fn guards() -> i32 {
	    let pair = (2, -2);
	    match pair {
		(x, y) if x == y => y,
		(x, y) if x + y == 0 => x,
		(x, _) if x % 2 == 1 => 0,
		_ => 0,
	    }
	}
}

#[test]                                                                                                                                                                                                      
fn test_pattern_m() {                                                                                                                                                                                       
    assert_eq!(guard::guards(), 2)                                                                                                                                                                        
} 


