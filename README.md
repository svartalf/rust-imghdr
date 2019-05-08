# imghdr

Library that determines the type of image contained in a file or byte stream, basically clone of the Python [imghdr](https://docs.python.org/library/imghdr.html) module.

[![Latest Version](https://img.shields.io/crates/v/imghdr.svg)](https://crates.io/crates/imghdr)
[![Latest Version](https://docs.rs/imghdr/badge.svg)](https://docs.rs/imghdr)
[![Rustc Version 1.31+](https://img.shields.io/badge/rustc-1.31+-lightgray.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)
[![Travis Build Status](https://travis-ci.org/svartalf/rust-imghdr.svg?branch=master)](https://travis-ci.org/svartalf/rust-imghdr)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)

## Examples

Check the file directly:

```rust
# extern crate imghdr;
# fn main() {
match imghdr::from_file("./tests/images/example.png") {
    Ok(Some(imghdr::Type::Png)) => println!("Yep, it is a PNG"),
    Ok(..) => println!("Nope, it is definitely not a PNG"),
    Err(e) => println!("Some error happened: {:?}", e),
}
# }
```

Or check the bytes stream:

```rust
# extern crate imghdr;
# use std::fs::File;
# use std::io::{self, Read};
#
# fn main() -> io::Result<()> {
let mut file = File::open("./tests/images/example.jpeg")?;
let mut content: Vec<u8> = vec![];
file.read_to_end(&mut content)?;

match imghdr::from_bytes(&content) {
    Some(imghdr::Type::Jpeg) => println!("And this is a Jpeg"),
    _ => println!("Can a Png, Bmp or other file format"),
}

# Ok(())
# }
```
