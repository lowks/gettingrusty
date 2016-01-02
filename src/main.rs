mod simple_listdir;
mod function_goodbye_world;
mod looping;
// mod write_to_file;
mod pattern_match;
mod semicolons;
mod string;
mod structures;
// mod guard_clause;

fn print_good_bye_world() {
    println!("Goodbye, cruel world!");
}

fn main() {
   let y = 10;
   print_good_bye_world(); 

   if y < 5 {
	println!("I won't show up");
   } else if  y == 10  {
   	print_good_bye_world(); 
   }
   
   function_goodbye_world::goodbye_world::goodbye_world::print_good_bye_world();
   simple_listdir::simple_listdir::main();
   looping::looping::main();
   pattern_match::pattern_m::main();
   semicolons::expr::expr();
   string::str::reverse();
   string::str::sort();
   string::str::replace();
   structures::struct_example::example();
}

