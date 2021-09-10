use image::{imageops::ColorMap, ImageBuffer, Pixel};

pub fn dither_floyd_steinberg<Pix, Map>(image: &mut ImageBuffer<Pix, Vec<u8>>, color_map: &Map)
where
    Map: ColorMap<Color = Pix> + ?Sized,
    Pix: Pixel<Subpixel = u8> + 'static,
{
    image::imageops::dither(image, color_map)
}

// based on this Python implementation, found here: http://mike.teczno.com/notes/atkinson.html
//
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
pub fn dither_atkinson<Pix, Map>(image: &mut ImageBuffer<Pix, Vec<u8>>, color_map: &Map)
where
    Map: ColorMap<Color = Pix> + ?Sized,
    Pix: Pixel<Subpixel = u8> + 'static,
{
    let width = image.width();
    let height = image.height();

    let mut mask: [[u32; 2]; 6];

    let mut error: [i16; 3];

    for y in 0..(height - 2) {
        for x in 1..(width - 2) {
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
                [x + 1, y],
                [x + 2, y],
                [x - 1, y + 1],
                [x, y + 1],
                [x + 1, y + 1],
                [x, y + 2],
            ];

            for [x, y] in mask {
                let pixel = image.get_pixel_mut(x, y);

                for (e, c) in error.iter().zip(pixel.channels_mut().iter_mut()) {
                    *c = match i16::from(*c) + e {
                        val if val < 0 => 0u8,
                        val if val > 0xFF => 0xFFu8,
                        val => val as u8,
                    }
                }
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
