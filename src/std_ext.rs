use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

use super::{patterns, Type};

/// Try to determine image format from an IO stream of bytes.
///
/// Available only with `std` feature (enabled by default).
///
/// ## Returns
///
///  * `Ok(Some(Type))` if it is a known image format
///  * `Ok(None)` if it is probably not an image
///  * `Err(..)` if failed to read minimal bytes amount for a guess
///
/// # Examples
///
/// ```rust
/// # use std::fs::File;
/// # use std::io::{Result, Read};
/// # fn main() -> Result<()> {
/// let mut file = File::open("./tests/images/example.png")?;
/// println!("{:?}", imghdr::from_reader(file));
/// # Ok(())
/// # }
/// ```
pub fn from_reader<T: Read>(mut f: T) -> Result<Option<Type>> {
    let mut buffer = [0; patterns::MAX_LENGTH];
    f.read_exact(&mut buffer)?;

    Ok(crate::from_bytes(&buffer))
}

/// Open file and try to determine if it is an image.
///
/// Available only with `std` feature (enabled by default).
///
/// # Errors
///
/// This function will return an `Err(std::io::Error)` if file is inaccessible or can't be read.
///
/// # Examples
///
/// ```rust
/// let result = imghdr::from_file("./tests/images/example.jpg");
///
/// println!("{:#?}", result);
/// ```
pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Option<Type>> {
    let file = File::open(path)?;

    from_reader(file)
}
