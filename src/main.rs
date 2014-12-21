fn print_good_bye_world() {
    println!("Goodbye, cruel world!");
}

fn main() {
   let y = 10i;

   if y < 5 {
	println!("I won't show up");
   } else if  y == 10  {
   	print_good_bye_world(); 
   }
}
