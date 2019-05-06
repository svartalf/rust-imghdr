use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use std::path::Path;

mod patterns;

/// Recognized image types
#[derive(Debug, PartialEq)]
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
    Ico
}


/// Tests the image data contained in the `f` bytes stream.
///
/// # Examples
///
/// ```rust,ignore
/// use std::fs::File;
/// use std::io::Read;
///
/// let mut file = File::open("/path/to/image.png").unwrap();
/// let mut content: Vec<u8> = vec![];
/// file.read_to_end(&mut content).unwrap();
/// println!("{:?}", imghdr::what(content.as_slice()));
/// ```
pub fn what<T: Read>(mut f: T) -> Option<Type> {
    let mut buffer = [0; 32];
    f.read(&mut buffer).unwrap();

    patterns::guess(buffer)
}


/// Open file and test if it an image.
///
/// # Errors
///
/// This function will return an `std::io::Error` if file is inaccessible or can't be read.
///
/// Also it will return an `std::io::Error` with a `std::io::ErrorKind::InvalidData` kind
/// if file is not an image.
///
/// # Examples
///
/// ```rust,ignore
/// imghdr.open("/path/to/image.jpg").unwrap();
/// ```
pub fn open<T: AsRef<Path>>(path: T) -> Result<Type, Error> {
    let mut file = try!(File::open(path));
    let mut buffer = [0; 32];
    try!(file.read_exact(&mut buffer));

    match patterns::guess(buffer) {
        Some(image) => Ok(image),
        None => Err(Error::new(ErrorKind::InvalidData, "Unknown file format"))
    }
}
