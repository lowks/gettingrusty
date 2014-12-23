use std::io::fs;

fn main() {
    let paths = fs::readdir(&Path::new("/tmp")).unwrap();

    for path in paths.iter() {
        println!("Name: {}", path.filename_str().unwrap())
    }
}
