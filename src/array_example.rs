pub mod array_example {
    use std::mem;
    pub fn main() {
        let xy: [i32; 5] = [1,2,3,4,5];
        println!("First element of array {}", xy[0]);
        println!("The length of the array is {}", xy.len());
        println!("array occupies {} bytes", mem::size_of_val(&xy));
    }
}
