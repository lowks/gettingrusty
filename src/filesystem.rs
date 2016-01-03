pub mod mkdir {

    use std::fs; 

    pub fn main() {
        match fs::create_dir("tmp") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {},
        }
        
        fs::remove_dir("tmp").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }
}

