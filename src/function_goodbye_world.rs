pub mod goodbye_world {
    pub mod goodbye_world {
        pub fn print_good_bye_world() -> &'static str {
	         "good bye world"
        }
    }
}

#[test]

fn test_for_goodbye_world() {
        assert_eq!(goodbye_world::goodbye_world::print_good_bye_world(), "good bye world");
}
