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

        for x in b.iter()
            .filter(|&x| *x > 2) { 
                println!("{} is bigger than 2 !", x)
            }

        for x in b.iter()
            .skip_while(|&x| *x < 4) { 
                println!("{} is less than 8 !", x)
            }

        for x in b.iter()
            .max() { 
                println!("{} is the max number !", x)
            }

        for x in b.iter()
            .min() { 
                println!("{} is the min number !", x)
            }


        // bit.ly/1KaDhWO
        println!("Using fold method to do product");
        
        let d = [2, 3, 4, 6, 8];

        let x = d.iter()
            .take(5)
            .fold(1, |a, b| a * b);
        println!("The product of d is {}", x);

    }
}
