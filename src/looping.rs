pub mod looping {
fn print_stuff() {

    for x in 1..15 {
        if x % 3 == 0 {
            println!("Number: {}", x);
        }
    }

    for (i, j) in (5..10).enumerate() {

        println!("i = {} and j = {}", i, j)

    }

    for x in 0..10 {
        if x % 3 == 0 { continue; }

        println!("Example using continue {}", x)
    }

	for x in 0..10 {
	    println!( "{}", x);
	}

        for _ in 0..4 {
            println!("Printing 5 times")
        }

	println!("Iter function loop");

	let values = vec![1, 2, 3, 4];
	for &x in values.iter() {
	    println!("{}", x);
	}
}
	pub fn main() {
	   print_stuff();
	}

}


