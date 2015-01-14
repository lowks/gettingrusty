#![feature(macro_rules)]

macro_rules! say_good_bye {
    () => {
        println!("Good Bye World!");
    }
}

fn main(){
    say_good_bye!()
}
