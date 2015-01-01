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
}
