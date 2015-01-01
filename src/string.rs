pub mod str {
	pub fn reverse() {
	let wordy: &'static str = "I am a hello world example";
		for word in wordy.words().rev() {
			print!("{}", word);
		}
	}

	pub fn sort() {
	let wordy: &'static str = "I am a hello world example";
		let mut chars: Vec<char> = wordy.chars().collect();		
		chars.sort();
		for char in chars.iter() {
			println!("{}", char);
		}	
	}

	pub fn replace() {
		let string_to_replace = String::from_str("Hello World!");
		let new_string: String = string_to_replace.replace("Hello", "Goodbye");
		println!("{} -> {}", string_to_replace, new_string);

	}
}
