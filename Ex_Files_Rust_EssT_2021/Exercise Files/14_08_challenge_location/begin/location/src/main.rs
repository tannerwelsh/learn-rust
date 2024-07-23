enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64), // lat, long
}

impl Location {
    fn display(&self) {
        match self {
            Location::Unknown => println!("Unknown location"),
            Location::Anonymous => println!("Nunya bizniss"),
            Location::Known(x, y) => println!("({x}, {y})"),
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}
