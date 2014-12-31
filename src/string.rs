pub mod str {
	pub fn reverse() {
		let wordy: &'static str = "I am a hello world example";
		for word in wordy.words().rev() {
			print!("{}", word);
		}
	}
}
