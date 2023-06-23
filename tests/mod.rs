#[cfg(test)]
use std::fs::File;
use std::io::Read;

extern crate imghdr;

macro_rules! assert_result {
    ($format:expr, $path:expr) => {{
        let result = imghdr::from_file($path).unwrap();

        assert_eq!($format, result);
    }
    {
        let file = File::open($path);
        if file.is_err() & $format.is_none() {
            return;
        } else {
            let mut content: Vec<u8> = vec![];
            file.unwrap().read_to_end(&mut content).unwrap();

            let result = imghdr::from_bytes(content.as_slice());

            assert_eq!($format, result);
        }
        /*
         */
    }};
}

#[test]
fn test_png() {
    assert_result!(Some(imghdr::Type::Png), "./tests/images/example.png");
}

#[test]
fn test_jpeg() {
    assert_result!(Some(imghdr::Type::Jpeg), "./tests/images/example.jpeg");
}

#[test]
fn test_jpg_raw() {
    assert_result!(Some(imghdr::Type::Jpeg), "./tests/images/example_raw.jpg");
}

#[test]
fn test_gif() {
    assert_result!(Some(imghdr::Type::Gif), "./tests/images/example.gif");
}

#[test]
fn test_webp() {
    assert_result!(Some(imghdr::Type::Webp), "./tests/images/example.webp");
}

#[test]
fn test_exr() {
    assert_result!(Some(imghdr::Type::Exr), "./tests/images/example.exr");
}

#[test]
fn test_bmp() {
    assert_result!(Some(imghdr::Type::Bmp), "./tests/images/example.bmp");
}

#[test]
fn test_tiff() {
    assert_result!(Some(imghdr::Type::Tiff), "./tests/images/example.tiff");
}

#[test]
fn test_xbm() {
    assert_result!(Some(imghdr::Type::Xbm), "./tests/images/example.xbm");
}

#[test]
fn test_bgp() {
    assert_result!(Some(imghdr::Type::Bgp), "./tests/images/example.bgp");
}

#[test]
fn test_pbm() {
    assert_result!(Some(imghdr::Type::Pbm), "./tests/images/example.pbm");
}

#[test]
fn test_pgm() {
    assert_result!(Some(imghdr::Type::Pgm), "./tests/images/example.pgm");
}

#[test]
fn test_ppm() {
    assert_result!(Some(imghdr::Type::Ppm), "./tests/images/example.ppm");
}

#[test]
fn test_rgb() {
    assert_result!(Some(imghdr::Type::Rgb), "./tests/images/example.rgb");
}

#[test]
fn test_rgbe() {
    assert_result!(Some(imghdr::Type::Rgbe), "./tests/images/example.hdr");
}

#[test]
fn test_flif() {
    assert_result!(Some(imghdr::Type::Flif), "./tests/images/example.flif");
}

#[test]
fn test_ico() {
    assert_result!(Some(imghdr::Type::Ico), "./tests/images/example.ico");
}

#[test]
fn test_not_a_image() {
    assert_result!(None::<imghdr::Type>, "./tests/images/not-a-image.txt");
}

#[test]
#[should_panic] // macro unwraps the `File::open` result
fn test_not_a_file() {
    assert_result!(None::<imghdr::Type>, "./tests/images/not-existing-file.foo");
}
