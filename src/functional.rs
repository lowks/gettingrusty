pub mod functional {

    pub fn product(input_array: &mut [i32]) -> i32 {
        let x = input_array.iter()
            .take(5)
            .fold(1, |a, b| a * b);
        x
    }

    pub fn min(input_array: &[i32]) -> i32 {
        let x = input_array
            .iter()
            .min()
            .unwrap();
        *x
    }

    pub fn count_elem(input_array: &[i32]) -> usize {
        let x = input_array
            .iter()
            .count();
        x
    }

    // pub fn factor(input_array: &mut [i32]) ->  std::iter::Filter::std::slice::Iter<i32> {
    //   fn (&&i32) -> bool {
    //     fn even(x: &&i32) -> bool { **x % 2 == 0 }
    //     let x = input_array
    //         .iter()
    //         .filter(even as for<'r> fn(&'r &_) -> _);
    //     x
    // }}

    pub fn main() {
        
        let a = [1, 2, 3, 4, 5];
        println!("\n\n====== Functional example ============");
        println!("Sum is {}", a.iter().fold(0, |sum, i| sum + i));
        // println!("Count of numbers {}", a.iter().count());
        println!("Count of elem is {} !", count_elem(&a));
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
            .filter(|&x| *x % 2 == 0) { 
                println!("{} is factor of 2 !", x)
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

        // assert_eq!(a.iter().all(|x| *x + 10));

        // for x in d.iter().all(|x| *x > 2)
           // {
           //     println!("{} is bigger than 2", x);
           // }

        // for x in d.iter().cycle() { println!("{}" ,x) }

        let mut input_array = [1, 2, 3, 4, 5, 6]; 
        println!("Product:: {}", product(&mut input_array));

        let mut input_array2 = [10, 9, 8, 7, 6, 5];
        println!("Min:: {}", min(&mut input_array2));

    }
}


#[test]

fn test_functional() {
    let mut input_array = [1, 1, 1, 1];
    assert_eq!(1, functional::product(&mut input_array));
    let mut input_array = [1, 2, 3, 4];
    assert_eq!(1, functional::min(&mut input_array));
    assert_eq!(4, functional::count_elem(&input_array));
}
