use crate::Type;

/// Maximum amount of bytes required for a format recognition
#[allow(dead_code)]
pub(crate) const MAX_LENGTH: usize = 12;

// Magic numbers
const PNG: &'static [u8] = b"\x89PNG\r\n\x1a\n";
const JFIF: &'static [u8] = b"JFIF";
const EXIF: &'static [u8] = b"Exif";
const JPEGRAW: &'static [u8] = b"\xDB\x00C"; // JPG RAW/Lossless without JFIF
const GIF87A: &'static [u8] = b"GIF87a";
const GIF89A: &'static [u8] = b"GIF89a";
const TIFF_MM: &'static [u8] = b"MM"; // Motorola byte order
const TIFF_II: &'static [u8] = b"II"; // Intel byte order
const RAST: &'static [u8] = b"\x59\xA6\x6A\x95";
const XBM: &'static [u8] = b"#define ";
const RIFF: &'static [u8] = b"RIFF";
const WEBP: &'static [u8] = b"WEBP";
const EXR: &'static [u8] = b"\x76\x2F\x31\x01";
const BMP: &'static [u8] = b"BM";
const BGP: &'static [u8] = b"BPG\xfb";
const RGB: &'static [u8] = b"\x01\xda";
const FLIF: &'static [u8] = b"FLIF";
const ICO: &'static [u8] = b"\x00\x00\x01\x00";

#[inline]
fn is_pbm(bytes: &[u8]) -> bool {
    if bytes.len() < 3 {
        return false;
    }

    // Safe, because length check was done before
    let a = unsafe { *bytes.get_unchecked(0) };
    let b = unsafe { *bytes.get_unchecked(1) };
    let c = unsafe { *bytes.get_unchecked(2) };

    a == b'P' && (b == b'1' || b == b'4') && (c == b' ' || c == b'\t' || c == b'\n' || c == b'\r')
}

#[inline]
fn is_pgm(bytes: &[u8]) -> bool {
    if bytes.len() < 3 {
        return false;
    }

    // Safe, because length check was done before
    let a = unsafe { *bytes.get_unchecked(0) };
    let b = unsafe { *bytes.get_unchecked(1) };
    let c = unsafe { *bytes.get_unchecked(2) };

    a == b'P' && (b == b'2' || b == b'5') && (c == b' ' || c == b'\t' || c == b'\n' || c == b'\r')
}

#[inline]
fn is_ppm(bytes: &[u8]) -> bool {
    if bytes.len() < 3 {
        return false;
    }

    // Safe, because length check was done before
    let a = unsafe { *bytes.get_unchecked(0) };
    let b = unsafe { *bytes.get_unchecked(1) };
    let c = unsafe { *bytes.get_unchecked(2) };

    a == b'P' && (b == b'3' || b == b'6') && (c == b' ' || c == b'\t' || c == b'\n' || c == b'\r')
}

#[inline]
fn is_rgbe(bytes: &[u8]) -> bool {
    &bytes[..8] == b"\x23\x3f\x52\x47\x42\x45\x0a\x47"
        || &bytes[..8] == b"\x23\x3f\x52\x41\x44\x49\x41\x4e"
}

pub fn guess(bytes: &[u8]) -> Option<Type> {
    match () {
        _ if &bytes[..8] == PNG => Some(Type::Png),
        _ if (&bytes[6..10] == JFIF) || (&bytes[6..10] == EXIF || (&bytes[3..6]) == JPEGRAW) => {
            Some(Type::Jpeg)
        }
        _ if (&bytes[..6] == GIF87A) || (&bytes[..6] == GIF89A) => Some(Type::Gif),
        _ if (&bytes[..2] == TIFF_MM) || (&bytes[..2] == TIFF_II) => Some(Type::Tiff),
        _ if &bytes[..4] == RAST => Some(Type::Rast),
        _ if &bytes[..8] == XBM => Some(Type::Xbm),
        _ if (&bytes[..4] == RIFF) && (&bytes[8..12] == WEBP) => Some(Type::Webp),
        _ if (&bytes[..4] == EXR) => Some(Type::Exr),
        _ if &bytes[..2] == BMP => Some(Type::Bmp),
        _ if &bytes[..4] == BGP => Some(Type::Bgp),
        _ if &bytes[..2] == RGB => Some(Type::Rgb),
        _ if &bytes[..4] == FLIF => Some(Type::Flif),
        _ if &bytes[..4] == ICO => Some(Type::Ico),
        _ if is_pbm(bytes) => Some(Type::Pbm),
        _ if is_pgm(bytes) => Some(Type::Pgm),
        _ if is_ppm(bytes) => Some(Type::Ppm),
        _ if is_rgbe(bytes) => Some(Type::Rgbe),
        _ => None,
    }
}
