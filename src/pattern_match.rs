pub mod pattern_m {
    pub fn main() {
        let x: int = 5;

        match x {
            5 => println!("Number is 5"),
            _ => println!("Some other number"),
        }

        match x {
            a @ 1...5 => println!("{} is within 1 to 5", a),
            _ => println!("Not within 1 to 5"),
        }
    }
}
