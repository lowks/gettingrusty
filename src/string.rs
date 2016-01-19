pub mod str {
	// http://stackoverflow.com/questions/27996430/reversing-a-string-in-rustlang-1-0-0/27996791#27996791
	pub fn reverse() -> String {
	let wordy = "lowks";
	let wordy_reverse = wordy
        .chars()
	    .rev()
	    // .flat_map(|g| g.chars())
	    .collect::<String>();
		// for word in wordy.words().rev() {
	        //		print!("{}", word);
		// }
	wordy_reverse
	}

        // pub fn backwords(word: str) -> String {
    pub fn backwords(word: String) -> String {
       word.chars().rev().collect()
    }

	pub fn sort() -> String {
	let wordy = "dabc";
		let mut chars: Vec<char> = wordy.chars().collect();		
		chars.sort();
                let mut output = String::new();
                for c in chars.into_iter() {
                    output.push(c);
                }
                output
		// for char in chars.iter() {
		//	println!("{}", char);
		//}	
	}

	pub fn replace() -> String {
		// let string_to_replace: &'static str = String::from_str("Hello World!");
        let string_to_replace = "Hello World!";
		let new_string = string_to_replace.replace("Hello", "Goodbye");
		format!("{} -> {}", string_to_replace, new_string)

	}
}

#[test]

fn test_replace() {
    assert_eq!("Hello World! -> Goodbye World!", str::replace());
    let reverse: String = str::reverse();
    let backwords  = str::backwords("lowks".to_string());
    assert_eq!("skwol", &*reverse);
    assert_eq!("skwol", &*backwords);
    assert_eq!("abcd", str::sort());   
}

#[test]

fn test_truncate() {
    let mut s = String::from("helloworld");
    s.truncate(5);
    assert_eq!("hello", s);
    assert_eq!(s.len(), 5);
}
