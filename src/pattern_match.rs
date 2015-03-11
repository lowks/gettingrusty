pub mod pattern_m {
    pub fn main() -> int {
        let x = 5;
        match x {
            5 => 5, 
            _ => 0,
        }
    }
}

#[test]

fn test_pattern_m() {
    assert_eq!(pattern_m::main(), 5)
}

