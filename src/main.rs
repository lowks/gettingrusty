mod listdir;
mod function_goodbye_world;
mod looping;
mod write_to_file;

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
   
   function_goodbye_world::goodbye_world::goodbye_world::print_good_bye_world();
   listdir::list::dir();
   looping::looping::main();
   write_to_file::write2file::main();
}
