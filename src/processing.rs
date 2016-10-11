extern crate image;

use self::image::{
    ImageBuffer,
    GenericImage,
    Luma,
    Pixel
};

pub fn sobel(frame : ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut result = ImageBuffer::new(640, 480);

    for j in 1..479 {
        for i in 1..639 {
            let north_west = frame[(i-1, j-1)].channels()[0] as i32;
            let north      = frame[(i, j-1)].channels()[0] as i32;
            let north_east = frame[(i+1, j-1)].channels()[0] as i32;

            let west = frame[(i-1, j)].channels()[0] as i32;
            let east = frame[(i+1, j)].channels()[0] as i32;

            let south_west = frame[(i-1, j+1)].channels()[0] as i32;
            let south      = frame[(i, j+1)].channels()[0] as i32;
            let south_east = frame[(i+1, j+1)].channels()[0] as i32;

            let gx : i32 = north_west + south_west + (2 * west)  - north_east - south_east - (2 * east);
            let gy : i32 = north_west + north_east + (2 * north) - south_west - south_east - (2 * south);

            let root : u8 = (((gx * gx) + (gy * gy)) as f32).sqrt() as u8;

            result.put_pixel(i, j, Luma([root]));
        }
    }

    return result;
}

pub fn sobel_optimized(frame : ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut result = ImageBuffer::new(640, 480);

    let mut j = 1;
    while j < 479 {
        let mut i = 1;
        while i < 638 {
            let north_west  = frame[(i-1, j-1)].channels()[0] as i32;
            let north       = frame[(i, j-1)].channels()[0] as i32;
            let north_east  = frame[(i+1, j-1)].channels()[0] as i32;
            let north_east2 = frame[(i+2, j-1)].channels()[0] as i32;

            let west  = frame[(i-1, j)].channels()[0] as i32;
            let west2 = frame[(i, j)].channels()[0] as i32;
            let east  = frame[(i+1, j)].channels()[0] as i32;
            let east2 = frame[(i+2, j)].channels()[0] as i32;

            let south_west  = frame[(i-1, j+1)].channels()[0] as i32;
            let south       = frame[(i, j+1)].channels()[0] as i32;
            let south_east  = frame[(i+1, j+1)].channels()[0] as i32;
            let south_east2 = frame[(i+2, j+1)].channels()[0] as i32;

            let gx : i32 = north_west + south_west + (west << 1)  - north_east - south_east - (east << 1);
            let gy : i32 = north_west + north_east + (north << 1) - south_west - south_east - (south << 1);

            let gx2 : i32 = north + (west2 << 1) + south - north_east2 - (east2 << 1) - south_east2;
            let gy2 : i32 = north + (north_east << 1) + north_east2 - south - (south_east << 1) - south_east2;

            let root  : u8 = (((gx.abs()  + gy.abs())  >> 1) as f32 * 1.414216) as u8;
            let root2 : u8 = (((gx2.abs() + gy2.abs()) >> 1) as f32 * 1.414216) as u8;

            result.put_pixel(i, j, Luma([root]));
            result.put_pixel(i + 1, j, Luma([root2]));

            i += 2;
        }
        j += 1;
    }

    return result;
}
