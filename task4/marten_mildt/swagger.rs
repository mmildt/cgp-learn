use std::fmt::{Display, Formatter, Result};

struct Swagger<T: Display> {
    disp: T,
}

impl<T: Display> Display for Swagger<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "#swag\n{}\n#yolo", self.disp)
    }
}

fn main() {
    let swag = Swagger { disp: "hallo" };

    println!("{}", swag);
}
