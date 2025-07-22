//  Declarative macros in Rust allow you to define reusable code patterns. Here's an example of a simple macro that calculates the square of a number:

macro_rules! square {
    ($x : expr) => {
        $x * $x
    };
}

fn main() {
    println!("Square is: {}" , square!(5));
}
