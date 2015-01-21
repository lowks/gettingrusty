pub mod pattern_m {
    pub fn main() -> int {
        let x = 5;

        match x {
            5 => 5, 
            _ => 0,
        }

        // match x {               
        //     a @ 1...5 => println!("{} is within 1 to 5", a),
        //     _ => println!("Not within 1 to 5"),
        //}
    }
}

#[test]

fn test_pattern_m() {
    assert_eq!(pattern_m::main(), 5)
}

