//! Library that determines the type of image contained in a file or byte stream,
//! basically the clone of the Python [imghdr](https://docs.python.org/library/imghdr.html) module.
//!
//! ## No-std support
//!
//! Can be used in `no-std` environments with disabled `std` feature (enabled by default).
//!
//!
//! ## Examples
//!
//! Check the file directly:
//!
//! ```rust
//! # extern crate imghdr;
//! # fn main() {
//! match imghdr::from_file("./tests/images/example.png") {
//!     Ok(Some(imghdr::Type::Png)) => println!("Yep, it is a PNG"),
//!     Ok(..) => println!("Nope, it is definitely not a PNG"),
//!     Err(e) => println!("Some error happened: {:?}", e),
//! }
//! # }
//! ```
//!
//! Or check the bytes stream:
//!
//! ```rust
//! # extern crate imghdr;
//! # use std::fs::File;
//! # use std::io::{self, Read};
//! #
//! # fn main() -> io::Result<()> {
//! let mut file = File::open("./tests/images/example.jpeg")?;
//! let mut content: Vec<u8> = vec![];
//! file.read_to_end(&mut content)?;
//!
//! match imghdr::from_bytes(&content) {
//!     Some(imghdr::Type::Jpeg) => println!("And this is a Jpeg"),
//!     _ => println!("Can a Png, Bmp or other file format"),
//! }
//!
//! # Ok(())
//! # }
//! ```
//!
//! It is not required to pass the fully read file into the crate functions,
//! right now `imghdr` requires only first 12 bytes of contents
//! for image format recognition.

#![cfg_attr(not(feature = "std"), no_std)]

mod patterns;

#[cfg(feature = "std")]
mod std_ext;

#[cfg(feature = "std")]
pub use self::std_ext::*;

/// Recognized image types
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    /// Gif 87a and 89a Files
    Gif,
    /// TIFF files
    Tiff,
    /// Sun Raster files
    Rast,
    /// X Bitmap files
    Xbm,
    /// JPEG data in JFIF or Exif formats
    Jpeg,
    /// BMP files
    Bmp,
    /// Portable Network Graphics
    Png,
    /// WebP files
    Webp,
    /// OpenEXR files
    Exr,
    /// BGP (Better Portable Graphics) files
    Bgp,
    /// PBM (Portable bitmap) files
    Pbm,
    /// PGM (Portable graymap) files
    Pgm,
    /// PPM (Portable pixmap) files
    Ppm,
    /// SGI image library files
    Rgb,
    /// RGBE (Radiance HDR) files
    Rgbe,
    /// FLIF (Free Lossless Image Format) files
    Flif,
    /// ICO files
    Ico,
    /// AVIF files
    Avif,
}

/// Try to determine image format from a bytes slice.
///
/// This function is available in a `no_std` environment.
///
/// ## Returns
///
/// `Some(Type)` if it is a known image format, `None` otherwise.
pub fn from_bytes<T: AsRef<[u8]>>(buf: T) -> Option<Type> {
    patterns::guess(buf.as_ref())
}
