
pub mod list {

    use std::io::fs;
    pub fn dir() {
        let paths = fs::readdir(&Path::new(".")).unwrap();

        for path in paths.iter() {
            println!("Name: {}", path.filename_str().unwrap())
        }
    }

}
