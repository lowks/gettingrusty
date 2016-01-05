pub mod array_example {
    pub fn main() {
        let xy: [i32; 5] = [1,2,3,4,5];
        println!("First element of array {}", xy[0]);
        println!("The length of the array is {}", xy.len());
    }
}
