use std::env;

extern crate imghdr;

fn main() {
    for path in env::args().skip(1) {
        println!("{:?}", imghdr::from_file(&path));
    }
}
