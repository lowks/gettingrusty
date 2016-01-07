pub mod functional {
    pub fn main() {
        let a = [1, 2, 3, 4, 5];
        println!("\n\n====== Functional example ============");
        println!("Sum is {}", a.iter().fold(0, |sum, i| sum + i));
        println!("Count of numbers {}", a.iter().count());
        for pair in "foo".chars().enumerate() {
            println!("{:?}", pair);
        }
    }
}
