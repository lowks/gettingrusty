#[cfg(target_os = "linux")]
fn on_linux() -> String{
    "You are on linux!".to_string()
}

#[cfg( not(target_os = "linux"))]
fn on_linux() -> String{
    "You not are on linux!".to_string()
}

fn main() {
    println!(on_linux());
}

#[test]

fn test_on_linux(){
    assert_eq!("You are on linux", on_linux());
}
