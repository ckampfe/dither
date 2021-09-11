use image::{imageops::ColorMap, ImageBuffer, Pixel};

macro_rules! some_if {
    ($cond:expr, $some:expr) => {
        if $cond {
            Some($some)
        } else {
            None
        }
    };
}

pub fn dither_floyd_steinberg<Pix, Map>(image: &mut ImageBuffer<Pix, Vec<u8>>, color_map: &Map)
where
    Map: ColorMap<Color = Pix> + ?Sized,
    Pix: Pixel<Subpixel = u8> + 'static,
{
    image::imageops::dither(image, color_map);

    #[cfg(debug_assertions)]
    assert_bilevel(image)
}

// import sys, PIL.Image
//
// img = PIL.Image.open(sys.argv[-1]).convert('L')
//
// threshold = 128*[0] + 128*[255]
//
// for y in range(img.size[1]):
//     for x in range(img.size[0]):
//
//         old = img.getpixel((x, y))
//         new = threshold[old]
//         err = (old - new) >> 3 # divide by 8
//
//         img.putpixel((x, y), new)
//
//         for nxy in [(x+1, y), (x+2, y), (x-1, y+1), (x, y+1), (x+1, y+1), (x, y+2)]:
//             try:
//                 img.putpixel(nxy, img.getpixel(nxy) + err)
//             except IndexError:
//                 pass
//
// img.show()
/// based on this Python implementation, found here: http://mike.teczno.com/notes/atkinson.html
pub fn dither_atkinson<Pix, Map>(image: &mut ImageBuffer<Pix, Vec<u8>>, color_map: &Map)
where
    Map: ColorMap<Color = Pix> + ?Sized,
    Pix: Pixel<Subpixel = u8> + 'static,
{
    let width = image.width();
    let height = image.height();

    let mut mask: [Option<[u32; 2]>; 6];

    let mut error: [i16; 3];

    for y in 0..height {
        for x in 0..width {
            let old_pixel = image[(x, y)];
            let new_pixel = image.get_pixel_mut(x, y);
            color_map.map_color(new_pixel);

            error = [0i16; 3];

            for ((e, &old), &new) in error
                .iter_mut()
                .zip(old_pixel.channels().iter())
                .zip(new_pixel.channels().iter())
            {
                *e = (i16::from(old) - i16::from(new)) / 8
            }

            mask = [
                some_if!(x + 1 < width, [x + 1, y]),
                some_if!(x + 2 < width, [x + 2, y]),
                some_if!(x.checked_sub(1).is_some() && y + 1 < height, [x - 1, y + 1]),
                some_if!(y + 1 < height, [x, y + 1]),
                some_if!(x + 1 < width && y + 1 < height, [x + 1, y + 1]),
                some_if!(y + 2 < height, [x, y + 2]),
            ];

            for [x, y] in mask.iter().flatten() {
                let pixel = image.get_pixel_mut(*x, *y);

                for (e, c) in error.iter().zip(pixel.channels_mut().iter_mut()) {
                    *c = match i16::from(*c) + e {
                        val if val < 0 => 0,
                        val if val > 255 => 255,
                        val => val as u8,
                    }
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    assert_bilevel(image)
}

///   * 2   The Sierra-2-4A filter
/// 1 1     (1/4)
pub fn dither_sierra_lite<Pix, Map>(image: &mut ImageBuffer<Pix, Vec<u8>>, color_map: &Map)
where
    Map: ColorMap<Color = Pix> + ?Sized,
    Pix: Pixel<Subpixel = u8> + 'static,
{
    let width = image.width();
    let height = image.height();

    let mut mask: [Option<([u32; 2], u8)>; 3];

    let mut error: [i16; 3];

    for y in 0..height {
        for x in 0..width {
            let old_pixel = image[(x, y)];
            let new_pixel = image.get_pixel_mut(x, y);
            color_map.map_color(new_pixel);

            error = [0i16; 3];

            for ((e, &old), &new) in error
                .iter_mut()
                .zip(old_pixel.channels().iter())
                .zip(new_pixel.channels().iter())
            {
                *e = (i16::from(old) - i16::from(new)) / 4
            }

            mask = [
                some_if!(x + 1 < width, ([x + 1, y], 2)),
                some_if!(
                    x.checked_sub(1).is_some() && y + 1 < height,
                    ([x - 1, y + 1], 1)
                ),
                some_if!(y + 1 < height, ([x, y + 1], 1)),
            ];

            for ([mask_x, mask_y], factor) in mask.iter().flatten() {
                let pixel = image.get_pixel_mut(*mask_x, *mask_y);

                for (e, c) in error.iter().zip(pixel.channels_mut().iter_mut()) {
                    *c = match i16::from(*c) + e * *factor as i16 {
                        val if val < 0 => 0,
                        val if val > 255 => 255,
                        val => val as u8,
                    }
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    assert_bilevel(image)
}

/// https://web.archive.org/web/20190316064436/http://www.efg2.com/Lab/Library/ImageProcessing/DHALF.TXT
pub fn dither_bayer<Pix, Map>(image: &mut ImageBuffer<Pix, Vec<u8>>, _color_map: &Map)
where
    Map: ColorMap<Color = Pix> + ?Sized,
    Pix: Pixel<Subpixel = u8> + 'static,
{
    let pattern: [[u8; 8]; 8] = [
        [0, 32, 8, 40, 2, 34, 10, 42],    /* 8x8 Bayer ordered dithering  */
        [48, 16, 56, 24, 50, 18, 58, 26], /* pattern.  Each input pixel   */
        [12, 44, 4, 36, 14, 46, 6, 38],   /* is scaled to the 0..63 range */
        [60, 28, 52, 20, 62, 30, 54, 22], /* before looking in this table */
        [3, 35, 11, 43, 1, 33, 9, 41],    /* to determine the action.     */
        [51, 19, 59, 27, 49, 17, 57, 25],
        [15, 47, 7, 39, 13, 45, 5, 37],
        [63, 31, 55, 23, 61, 29, 53, 21],
    ];

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        for channel in pixel.channels_mut() {
            let scaled = *channel >> 2;
            if scaled > pattern[x as usize & 7][y as usize & 7] {
                *channel = 255;
            } else {
                *channel = 0;
            }
        }
    }

    #[cfg(debug_assertions)]
    assert_bilevel(image)
}

//     grayscaleImage.mapSelf(brightness =>
//   brightness + (Math.random() - 0.5) > 0.5
//     ? 1.0
//     : 0.0
// );
/// https://surma.dev/things/ditherpunk/
pub fn dither_random_threshold<Pix, Map>(image: &mut ImageBuffer<Pix, Vec<u8>>, _color_map: &Map)
where
    Map: ColorMap<Color = Pix> + ?Sized,
    Pix: Pixel<Subpixel = u8> + 'static,
{
    for pixel in image.pixels_mut() {
        let r: u8 = rand::random();
        for channel in pixel.channels_mut() {
            *channel = if i16::from(*channel) + (r as i16 - 127i16) > 127 {
                255
            } else {
                0
            };
        }
    }

    #[cfg(debug_assertions)]
    assert_bilevel(image)
}

#[cfg(debug_assertions)]
fn assert_bilevel<Pix>(image: &mut ImageBuffer<Pix, Vec<u8>>)
where
    Pix: Pixel<Subpixel = u8> + 'static,
{
    for (x, y, pixel) in image.enumerate_pixels() {
        for channel in pixel.channels() {
            debug_assert!(
                *channel == 0 || *channel == 255,
                "x={}, y={}, channel={}",
                x,
                y,
                channel
            )
        }
    }
}
