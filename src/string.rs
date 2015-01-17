pub mod str {
	// http://stackoverflow.com/questions/27996430/reversing-a-string-in-rustlang-1-0-0/27996791#27996791
	pub fn reverse() -> String {
	let wordy: &str = "lowks";
	let wordy_reverse: String = wordy
	    .graphemes(true)
	    .rev()
	    // .flat_map(|g| g.chars())
	    .collect();
		// for word in wordy.words().rev() {
	        //		print!("{}", word);
		// }
	wordy_reverse
	}

	pub fn sort() {
	let wordy: &'static str = "I am a hello world example";
		let mut chars: Vec<char> = wordy.chars().collect();		
		chars.sort();
		for char in chars.iter() {
			println!("{}", char);
		}	
	}

	pub fn replace() -> String {
		let string_to_replace = String::from_str("Hello World!");
		let new_string: String = string_to_replace.replace("Hello", "Goodbye");
		format!("{} -> {}", string_to_replace, new_string)

	}
}

#[test]

fn test_replace() {
	assert_eq!("Hello World! -> Goodbye World!", str::replace());
	let reverse: String = str::reverse();
	assert_eq!("skwol", &*reverse);
}
