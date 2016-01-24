struct Implement {
   x: &'static str,
   y: &'static str,
}

impl Implement {

    fn new() -> Implement {
        Implement {x: "hoho", y: "haha"}
    }

    fn print_helloworld(&self) {
        println!("{:?}", self.x);
    }

    fn print_goodbyeworld(&self) {
        println!("{:?}", self.y);
    }
}

fn main() {
    // let i = Implement {x: "hoho", y: "haha"};
    let i = Implement::new();

    i.print_helloworld();
    i.print_goodbyeworld();
}
