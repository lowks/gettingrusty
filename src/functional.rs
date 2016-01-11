pub mod functional {
    pub fn main() {

        let a = [1, 2, 3, 4, 5];
        println!("\n\n====== Functional example ============");
        println!("Sum is {}", a.iter().fold(0, |sum, i| sum + i));
        println!("Count of numbers {}", a.iter().count());
        for pair in "foo".chars().enumerate() {
            println!("{:?}", pair);
        }

        let strs = ["hello", "world", "yo!"];

        for (i, s) in strs.iter().enumerate() {
            println!("{} {}", i, s);
        }

        for item in strs.iter() {
            println!("### {}", item);
        }

        for item in 0..strs.len() {
            println!("%%% {}", strs[item].to_uppercase());
        }

        let a = [1, 2, 3, 4, 5, 6, 7, 8];

        for x in a.iter()
            .map(|&x| x + 10) {
            println!("After mapping and adding 10 to list {}", x);
        }

        let b = [2, 3, 4, 6, 8];

        for x in b.iter()
            .skip(3)
            .take(1) {
                println!("This is skipped {}", x)
            }
    }
}
