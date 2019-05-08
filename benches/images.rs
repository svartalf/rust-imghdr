#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use imghdr;

static BGP: &'static [u8] = include_bytes!("../tests/images/example.bgp");
static BMP: &'static [u8] = include_bytes!("../tests/images/example.bmp");
static EXR: &'static [u8] = include_bytes!("../tests/images/example.exr");
static FLIF: &'static [u8] = include_bytes!("../tests/images/example.flif");
static GIF: &'static [u8] = include_bytes!("../tests/images/example.gif");
static HDR: &'static [u8] = include_bytes!("../tests/images/example.hdr");
static ICO: &'static [u8] = include_bytes!("../tests/images/example.ico");
static JPEG: &'static [u8] = include_bytes!("../tests/images/example.jpeg");
static PBM: &'static [u8] = include_bytes!("../tests/images/example.pbm");
static PGM: &'static [u8] = include_bytes!("../tests/images/example.pgm");
static PNG: &'static [u8] = include_bytes!("../tests/images/example.png");
static PPM: &'static [u8] = include_bytes!("../tests/images/example.ppm");
static RGB: &'static [u8] = include_bytes!("../tests/images/example.rgb");
static TIFF: &'static [u8] = include_bytes!("../tests/images/example.tiff");
static WEBP: &'static [u8] = include_bytes!("../tests/images/example.webp");
static XBM: &'static [u8] = include_bytes!("../tests/images/example.xbm");

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bgp", |b| b.iter(|| imghdr::from_bytes(black_box(&BGP))));
    c.bench_function("bmp", |b| b.iter(|| imghdr::from_bytes(black_box(&BMP))));
    c.bench_function("exr", |b| b.iter(|| imghdr::from_bytes(black_box(&EXR))));
    c.bench_function("flif", |b| b.iter(|| imghdr::from_bytes(black_box(&FLIF))));
    c.bench_function("gif", |b| b.iter(|| imghdr::from_bytes(black_box(&GIF))));
    c.bench_function("hdr", |b| b.iter(|| imghdr::from_bytes(black_box(&HDR))));
    c.bench_function("ico", |b| b.iter(|| imghdr::from_bytes(black_box(&ICO))));
    c.bench_function("jpeg", |b| b.iter(|| imghdr::from_bytes(black_box(&JPEG))));
    c.bench_function("pbm", |b| b.iter(|| imghdr::from_bytes(black_box(&PBM))));
    c.bench_function("pgm", |b| b.iter(|| imghdr::from_bytes(black_box(&PGM))));
    c.bench_function("png", |b| b.iter(|| imghdr::from_bytes(black_box(&PNG))));
    c.bench_function("ppm", |b| b.iter(|| imghdr::from_bytes(black_box(&PPM))));
    c.bench_function("rgb", |b| b.iter(|| imghdr::from_bytes(black_box(&RGB))));
    c.bench_function("tiff", |b| b.iter(|| imghdr::from_bytes(black_box(&TIFF))));
    c.bench_function("webp", |b| b.iter(|| imghdr::from_bytes(black_box(&WEBP))));
    c.bench_function("xbm", |b| b.iter(|| imghdr::from_bytes(black_box(&XBM))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
