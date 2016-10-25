extern crate image;

use self::image::{
    ImageBuffer,
    Luma,
    Pixel
};

pub fn sobel(frame : ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut result = ImageBuffer::new(640, 480);

    for i in 1..638 {
        for j in 1..478 {
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

    let mut i = 1;
    while i < 638 {
        let mut j = 1;
        while j < 479 {
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

            j += 1;
        }
        i += 2;
    }

    return result;
}

pub fn sobel_and_threshold(frame : ImageBuffer<Luma<u8>, Vec<u8>>, threshold : u8) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut result = ImageBuffer::new(640, 480);

    let mut i = 1;
    while i < 638 {
        let mut j = 1;
        while j < 479 {
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
            let root =
                if root > threshold {
                    255
                } else {
                    0
                };

            let root2 : u8 = (((gx2.abs() + gy2.abs()) >> 1) as f32 * 1.414216) as u8;
            let root2 =
                if root2 > threshold {
                    255
                } else {
                    0
                };

            result.put_pixel(i, j, Luma([root]));
            result.put_pixel(i + 1, j, Luma([root2]));

            j += 1;
        }
        i += 2;
    }

    return result;
}

pub fn median_filter(frame : ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut result = ImageBuffer::new(640, 480);

    let mut kernel = [0; 9];

    for i in 1..638 {
        for j in 1..478 {
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
    values : [u16 ; 256],
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

        while sum > 0 && index < 256 {
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

    for i in 0..640 {
        for j in 0..480 {

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


    for i in 0..640 {
        let mut hist = Histogram::new();

        for k in (i as i32 - kernel_offset as i32)..(i as i32 + kernel_offset as i32 + 1) {
            for l in 0..(kernel_offset + 1) {
                if check_coordinates(k, l as i32) {
                    let color = frame[(k as u32, l as u32)].channels()[0];
                    hist.increment(color);
                }
            }
        }

        for j in 0..480 {
            let old_column_coord = j as i32 - kernel_offset as i32 - 1i32;
            let new_column_coord = j as i32 + kernel_offset as i32;

            for k in (i as i32 - kernel_offset as i32)..(i as i32 + kernel_offset as i32 + 1) {
                if check_coordinates(k, old_column_coord) {
                    let color = frame[(k as u32, old_column_coord as u32)].channels()[0];
                    hist.decrement(color);
                }
            }

            for k in (i as i32 - kernel_offset as i32)..(i as i32 + kernel_offset as i32 + 1) {
                if check_coordinates(k, new_column_coord) {
                    let color = frame[(k as u32, new_column_coord as u32)].channels()[0];
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

#[cfg(test)]
mod tests {
    use ::test::Bencher;
    use image::{
        ConvertBuffer,
        ImageBuffer,
        Luma
    };
    use ::capture;

    #[test]
    fn check_coordinates() {
        use super::check_coordinates;

        assert!(!check_coordinates(-1, 0));
        assert!(!check_coordinates(0, -1));
        assert!(!check_coordinates(-1, -1));

        assert!(check_coordinates(0, 0));
        assert!(check_coordinates(42, 0));
        assert!(check_coordinates(0, 42));
        assert!(check_coordinates(42, 42));

        assert!(!check_coordinates(640, 0));
        assert!(!check_coordinates(640, 42));
        assert!(!check_coordinates(0, 480));
        assert!(!check_coordinates(42, 480));
    }

    #[bench]
    fn sobel(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::sobel(frame)
        });
    }

    #[bench]
    fn sobel_optimized(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::sobel_optimized(frame)
        });
    }

    #[bench]
    fn sobel_and_threshold(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::sobel_and_threshold(frame, 127)
        });
    }

    #[bench]
    fn median_filter(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::median_filter(frame)
        });
    }

    #[bench]
    fn median_filter_hist_3(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::median_filter_hist(frame, 3)
        });
    }

    #[bench]
    fn median_filter_hist_5(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::median_filter_hist(frame, 5)
        });
    }

    #[bench]
    fn median_filter_hist_9(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::median_filter_hist(frame, 9)
        });
    }

    #[bench]
    fn median_filter_hist_15(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::median_filter_hist(frame, 15)
        });
    }

    #[bench]
    fn median_filter_hist_21(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::median_filter_hist(frame, 21)
        });
    }

    #[bench]
    fn median_filter_hist_31(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::median_filter_hist(frame, 31)
        });
    }

    #[bench]
    fn median_filter_hist_optimized_3(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::median_filter_hist_optimized(frame, 3)
        });
    }

    #[bench]
    fn median_filter_hist_optimized_5(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::median_filter_hist_optimized(frame, 5)
        });
    }

    #[bench]
    fn median_filter_hist_optimized_9(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::median_filter_hist_optimized(frame, 9)
        });
    }

    #[bench]
    fn median_filter_hist_optimized_15(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::median_filter_hist_optimized(frame, 15)
        });
    }

    #[bench]
    fn median_filter_hist_optimized_21(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::median_filter_hist_optimized(frame, 21)
        });
    }

    #[bench]
    fn median_filter_hist_optimized_31(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::median_filter_hist_optimized(frame, 31)
        });
    }

}
