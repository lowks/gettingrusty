pub mod write2file {
    pub fn main() {
	use std::old_io::File;
	let path = Path::new("foo.txt");
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("Coult not create bro! {} {}", display, why.desc),
            Ok(file) => file,
        };

	match file.write(b"Merry Christmas hoho!") {
            Err(why) => panic!("Coult not write to file bro! {} {}", display, why.desc),
            Ok(_) => println!("Successfully written to file"),
        };
        

        let mut file_to_read = match File::open(&Path::new("foo.txt")) {
            Err(why) => panic!("File not found bro! {} {}", display, why.desc),
            Ok(file) => file,
        };


        match file_to_read.read_to_string() {
            Err(why) => panic!("File can't be read bro! {} {}", display, why.desc),
            Ok(string) => println!("{} contains this {}", display, string),
        }
    }
}
