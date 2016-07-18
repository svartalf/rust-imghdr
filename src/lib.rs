use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use std::path::Path;

/// Recognized image types
#[derive(Debug, PartialEq)]
pub enum Type {
    /// SGI ImgLib files
    Rgb,
    /// Gif 87a and 89a Files
    Gif,
    /// Portable Bitmap files
    Pbm,
    /// Portable Graymap files
    Pgm,
    /// Portable Pixmap files
    Ppm,
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
}

// Magic numbers
const PNG: &'static [u8] = b"\x89PNG\r\n\x1a\n";
const JFIF: &'static [u8] = b"JFIF";
const EXIF: &'static [u8] = b"Exif";
const GIF87A: &'static [u8] = b"GIF87a";
const GIF89A: &'static [u8] = b"GIF89a";
const TIFF_MM: &'static [u8] = b"MM"; // Motorola byte order
const TIFF_II: &'static [u8] = b"II"; // Intel byte order
const RGB: &'static [u8] = b"\x01\xda";
const RAST: &'static [u8] = b"\x59\xA6\x6A\x95";
const XBM: &'static [u8] = b"#define ";
const RIFF: &'static [u8] = b"RIFF";
const WEBP: &'static [u8] = b"WEBP";
const BMP: &'static [u8] = b"BM";


fn guess(ref bytes: [u8; 32]) -> Option<Type> {
    match () {
        _ if &bytes[0..8] == PNG => Some(Type::Png),
        _ if (&bytes[6..10] == JFIF) | (&bytes[6..10] == EXIF) => Some(Type::Jpeg),
        _ if (&bytes[..6] == GIF87A) | (&bytes[..6] == GIF89A) => Some(Type::Gif),
        _ if (&bytes[..2] == TIFF_MM) | (&bytes[..2] == TIFF_II) => Some(Type::Tiff),
        _ if &bytes[0..2] == RGB => Some(Type::Rgb),
        _ if &bytes[0..4] == RAST => Some(Type::Rast),
        _ if &bytes[0..8] == XBM => Some(Type::Xbm),
        _ if (&bytes[0..4] == RIFF) & (&bytes[8..12] == WEBP) => Some(Type::Webp),
        _ if &bytes[0..2] == BMP => Some(Type::Bmp),
        _ => None
    }
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

    guess(buffer)
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

    match guess(buffer) {
        Some(image) => Ok(image),
        None => Err(Error::new(ErrorKind::InvalidData, "Unknown file format"))
    }
}

