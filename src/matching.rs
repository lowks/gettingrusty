pub mod matches {
    pub fn main() {

        let x = 1;
        match x {
            1 => println!("I am 1"),
            2 => println!("You will never see me because no match!"),
            _ => println!("Alamak!"),
        }

        let y = 'a';
        match y {
            x => println!("ho ho ho"),
        }
        println!("X is ... {}", x)
    }
}
