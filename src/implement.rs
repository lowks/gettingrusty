struct Implement {
   x: &'static str,
   y: &'static str,
}

impl Implement {
    fn print_helloworld(&self) {
        println!("{:?}", self.x);
    }

    fn print_goodbyeworld(&self) {
        println!("{:?}", self.y);
    }
}

fn main() {
    let i = Implement {x: "hoho", y: "haha"};
    i.print_helloworld();
    i.print_goodbyeworld();
}
