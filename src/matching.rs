pub mod matches {
    pub fn main() {
        let x = 1;
        match x {
            1 => println!("I am 1"),
            2 => println!("You will never see me because no match!"),
            _ => println!("Alamak!"),
        }

	match x + 2 {
	    3 => println!("Hey ! 1 + 2 is actually 3 !"),
	    _ => println!("This will never get printed"),
	}

        let y = 25;

        match y % 5 {
            0 => println!("y is divisible by 5!"),
            _ => println!("y is NOT divisible by 5"),
        }

	let z = "a A Word";

	let k = z.len() + 5;

	println!("The value of k is {}", k);
    }
}
