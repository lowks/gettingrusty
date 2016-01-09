pub mod tuple {
    pub fn run() {
        println!("\n\n#### Tuple Example ####");
        let long_tuple = (1u8, 2u16, 3u32, 4u64,
                          -1i8, -2i16, -3i32, -4i64,
                          0.1f32, 0.2f64, 'a', true);
        println!("Long tuple first item {}", long_tuple.0);
        let tuple = (1, "hello", 4.5, true);
        let (a, b, c, d) = tuple;
        println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    }
}
