pub mod matches {

    pub fn multiple_match(a: i32) -> i32 {
       match a { 
            1 | 2 => return 0,
            _ => return 10,
       } 
    }
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
    println!("The value of a is {}", multiple_match(1));
    }
}

#[test]

fn test_matching() {
    assert_eq!(matches::multiple_match(1), 0);
}
