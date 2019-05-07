# imghdr

Library that determines the type of image contained in a file or byte stream, basically clone of the Python [imghdr](https://docs.python.org/library/imghdr.html) module.

[![Latest Version](https://img.shields.io/crates/v/imghdr.svg)](https://crates.io/crates/imghdr)
[![Latest Version](https://docs.rs/imghdr/badge.svg)](https://docs.rs/imghdr)
[![Travis Build Status](https://travis-ci.org/svartalf/rust-imghdr.svg?branch=master)](https://travis-ci.org/svartalf/rust-imghdr)
[![AppVeyor Build status](https://ci.appveyor.com/api/projects/status/b76fds0oafhyj8gi?svg=true)](https://ci.appveyor.com/project/svartalf/rust-imghdr)
![License](http://img.shields.io/:license-mit-blue.svg)
[![Crates.io](https://img.shields.io/badge/crates.io-imghdr-green.svg)](https://crates.io/crates/imghdr)

## Examples

Check a file directly:

```rust
extern crate imghdr;

fn main() {
    match imghdr::open("/path/to/image.png") {
        Ok(imghdr::Type::Png) => println!("Yep, it is a PNG"),
        _ => println!("Nope, it is definitely not a PNG"),
    }
}
```

Or check a byte stream:

```rust
extern crate imghdr;

fn main() {
    let mut file = File::open("/path/to/image.png").unwrap();
    let mut content: Vec<u8> = vec![];
    file.read_to_end(&mut content).unwrap();

    match imghdr::what(content.as_slice()) {
        Some(imghdr::Type::Jpg) => println!("And this is a Jpeg"),
        _ => println!("Can a Png, Bmp or other file format"),
    }
}
```