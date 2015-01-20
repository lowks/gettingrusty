
pub mod list {

    use std::io::fs;
    use std::io::fs::PathExtensions;

    pub fn dir() {
        let paths = fs::readdir(&Path::new(".")).unwrap();

        for path in paths.iter() {
            println!("Name: {}", path.filename_str().unwrap())
        }

        let path2 = Path::new("/tmp");

        println!("File Size: {}", path2.stat().unwrap().size);
        println!("Is dir ? : {}", path2.is_dir());

    }

    pub fn does_it_exist(path_input: &str) -> String {
        let path3 = Path::new(path_input);  
        match path3.exists() {
            true => "Exists!".to_string(),
            false => "Does not!".to_string(),
        }
    }

    pub fn is_it_file(input_file: &str) -> String {
        let path3 = Path::new(input_file);  
        match path3.is_file() {
            true => "File!".to_string(),
            false => "Not file!".to_string(),
        }
    }

    pub fn is_it_file_or_dir(input_file: &str) -> String {
        let path3 = Path::new(input_file);  
        match path3.is_file() {
            
            true => "File!".to_string(),
            false => match path3.is_dir() {

                true => "Dir!".to_string(),
                _ => "Dont care!".to_string(),

            }
        }
    }
    
}


#[test]

fn test_path(){
    assert_eq!(list::does_it_exist("/tmp"), "Exists!");
    assert_eq!(list::is_it_file("/tmp"), "Not file!");
    assert_eq!(list::is_it_file_or_dir("/tmp"), "Dir!");
}
