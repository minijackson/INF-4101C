extern crate image;

use self::image::{
    ImageBuffer,
    Luma,
    Pixel
};

pub fn sobel(frame : ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut result = ImageBuffer::new(640, 480);

    for j in 1..478 {
        for i in 1..638 {
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

pub fn median_filter(frame : ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut result = ImageBuffer::new(640, 480);

    let mut kernel = [0; 9];

    for j in 1..478 {
        for i in 1..638 {
            // Fill kernel
            for k in 0..3 {
                for l in 0..3 {
                    let index = k + 3 * l;
                    let coord_x = (i + k - 1) as u32;
                    let coord_y = (j + l - 1) as u32;

                    kernel[index] = frame[(coord_x, coord_y)].channels()[0];
                }
            }

            kernel.sort();
            let pixel_value = kernel[5];
            result.put_pixel(i as u32, j as u32, Luma([pixel_value]));
        }
    }

    return result;
}

struct Histogram {
    values : [u8 ; 256],
    count  : u32
}

impl Histogram {
    pub fn new() -> Histogram {
        Histogram {
            values : [0 ; 256],
            count  : 0
        }
    }

    pub fn increment(&mut self, luma : u8) {
        self.values[luma as usize] += 1;
        self.count += 1;
    }

    pub fn decrement(&mut self, luma : u8) {
        self.values[luma as usize] -= 1;
        self.count -= 1;
    }

    pub fn median(&self) -> u8 {
        //assert!(self.count != 0, "Attempt to get median value of empty histogram");

        let mut sum : i32 = self.count as i32 / 2;
        let mut index = 0;

        while sum > 0 {
            sum -= self.values[index] as i32;
            index += 1;
        }

        return (index - 1) as u8;
    }
}

pub fn median_filter_hist(frame : ImageBuffer<Luma<u8>, Vec<u8>>, kernel_size : usize) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    //assert!(kernel_size % 2 == 1, "Kernel size must be odd.");

    let mut result = ImageBuffer::new(640, 480);

    let kernel_offset = (kernel_size - 1) / 2;

    for j in 0..480 {
        for i in 0..640 {

            let mut hist = Histogram::new();

            for k in (i as i32 - kernel_offset as i32)..(i as i32 + kernel_offset as i32 + 1) {
                for l in (j as i32 - kernel_offset as i32)..(j as i32 + kernel_offset as i32 + 1) {
                    if 0 <= k && k < 640 && 0 <= l && l < 480 {
                        let color = frame[(k as u32, l as u32)].channels()[0];
                        hist.increment(color);
                    }
                }
            }

            let median_color = Luma([hist.median()]);
            result.put_pixel(i as u32, j as u32, median_color);
        }
    }

    return result;
}

pub fn median_filter_hist_optimized(frame : ImageBuffer<Luma<u8>, Vec<u8>>, kernel_size : usize) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    //assert!(kernel_size % 2 == 1, "Kernel size must be odd");

    let mut result = ImageBuffer::new(640, 480);

    let kernel_offset = (kernel_size - 1) / 2;

    for j in 0..480 {

        let mut hist = Histogram::new();

        for k in (j as i32 - kernel_offset as i32)..(j as i32 + kernel_offset as i32 + 1) {
            for l in 0..(kernel_offset + 1) {
                if check_coordinates(l as i32, k) {
                    let color = frame[(l as u32, k as u32)].channels()[0];
                    hist.increment(color);
                }
            }
        }

        for i in 0..640 {
            let old_column_coord = i as i32 - kernel_offset as i32 - 1i32;
            let new_column_coord = i as i32 + kernel_offset as i32;

            for k in (j as i32 - kernel_offset as i32)..(j as i32 + kernel_offset as i32 + 1) {
                if check_coordinates(old_column_coord, k) {
                    let color = frame[(old_column_coord as u32, k as u32)].channels()[0];
                    hist.decrement(color);
                }
            }

            for k in (j as i32 - kernel_offset as i32)..(j as i32 + kernel_offset as i32 + 1) {
                if check_coordinates(new_column_coord, k) {
                    let color = frame[(new_column_coord as u32, k as u32)].channels()[0];
                    hist.increment(color);
                }
            }

            let median_color = Luma([hist.median()]);
            result.put_pixel(i as u32, j as u32, median_color);
        }
    }

    return result;
}

fn check_coordinates(x : i32, y : i32) -> bool {
    return 0 <= x && x < 640 && 0 <= y && y < 480;
}
