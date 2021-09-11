use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use image::ImageBuffer;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

fn floyd_steinberg(c: &mut Criterion) {
    let img = ImageBuffer::from_fn(WIDTH, HEIGHT, |x, _y| {
        if x % 2 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    // c.bench_function("floyd-steinberg", |b| {
    //     b.iter(|| dither::dither_floyd_steinberg(&mut img, &image::imageops::BiLevel))
    // });
    c.bench_function("floyd-steinberg", move |b| {
        b.iter_batched(
            || img.clone(),
            |mut i| dither::dither_floyd_steinberg(&mut i, &image::imageops::BiLevel),
            BatchSize::SmallInput,
        )
    });
}

fn atkinson(c: &mut Criterion) {
    let img = ImageBuffer::from_fn(WIDTH, HEIGHT, |x, _y| {
        if x % 2 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    c.bench_function("atkinson", move |b| {
        b.iter_batched(
            || img.clone(),
            |mut i| dither::dither_atkinson(&mut i, &image::imageops::BiLevel),
            BatchSize::SmallInput,
        )
    });
}

fn sierra_lite(c: &mut Criterion) {
    let img = ImageBuffer::from_fn(WIDTH, HEIGHT, |x, _y| {
        if x % 2 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    c.bench_function("sierra lite", move |b| {
        b.iter_batched(
            || img.clone(),
            |mut i| dither::dither_sierra_lite(&mut i, &image::imageops::BiLevel),
            BatchSize::SmallInput,
        )
    });
}

fn bayer(c: &mut Criterion) {
    let img = ImageBuffer::from_fn(WIDTH, HEIGHT, |x, _y| {
        if x % 2 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    c.bench_function("bayer", move |b| {
        b.iter_batched(
            || img.clone(),
            |mut i| dither::dither_bayer(&mut i, &image::imageops::BiLevel),
            BatchSize::SmallInput,
        )
    });
}

fn random_threshold(c: &mut Criterion) {
    let img = ImageBuffer::from_fn(WIDTH, HEIGHT, |x, _y| {
        if x % 2 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    c.bench_function("random threshold", move |b| {
        b.iter_batched(
            || img.clone(),
            |mut i| dither::dither_random_threshold(&mut i, &image::imageops::BiLevel),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    floyd_steinberg,
    atkinson,
    sierra_lite,
    bayer,
    random_threshold
);
criterion_main!(benches);
