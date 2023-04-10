use crate::ppm::{PPM, Pixel};

pub fn rotate_left(image: PPM) -> PPM {
    // create a new PPM object to represent the rotated image
    let mut rotated_image = PPM::new();

    // copy header information
    rotated_image.magic = image.magic.clone();
    rotated_image.maxc = image.maxc;
    rotated_image.width = image.height;
    rotated_image.height = image.width;

    // assign the pixel array for the rotated image
    rotated_image.pixels = vec![Pixel::new(); image.pixel_count() as usize];

    // loop through each pixel in the new image
    for x in 0..image.width {
        for y in 0..image.height {

            let current_pixel_pos = y * image.width + x; // get the current pixel position in the 1d array

            // calculate the x and y value for the pixel on the new rotated image
			let new_x = y;
			let new_y = rotated_image.height - x - 1;

            // calculate the pixel position in the 1d array for the pixel on the new rotated image
			let rotated_pixel_pos = new_y * rotated_image.width + new_x;
            rotated_image.pixels[rotated_pixel_pos as usize] = image.pixels[current_pixel_pos as usize]; // copy pixel to the new location
        }
    }

    return rotated_image;
}

pub fn rotate_right(image: PPM) -> PPM {
    // create a new PPM object to represent the rotated image
    let mut rotated_image = PPM::new();

    // copy header information
    rotated_image.magic = image.magic.clone();
    rotated_image.maxc = image.maxc;
    rotated_image.width = image.height;
    rotated_image.height = image.width;

    // assign the pixel array for the rotated image
    rotated_image.pixels = vec![Pixel::new(); image.pixel_count() as usize];

    // loop through each pixel in the new image
    for x in 0..image.width {
        for y in 0..image.height {

            let current_pixel_pos = y * image.width + x; // get the current pixel position in the 1d array

            // calculate the x and y value for the pixel on the new rotated image
			let new_x = rotated_image.width - y - 1;
			let new_y = x;

            // calculate the pixel position in the 1d array for the pixel on the new rotated image
			let rotated_pixel_pos = new_y * rotated_image.width + new_x;
            rotated_image.pixels[rotated_pixel_pos as usize] = image.pixels[current_pixel_pos as usize]; // copy pixel to the new location
        }
    }

    return rotated_image;
}

pub fn half_size(image: PPM) -> PPM {
    let mut half_image = PPM::new();

    // copy header info
    half_image.magic = image.magic.clone();
    half_image.maxc = image.maxc;
    half_image.width = image.width / 2;
    half_image.height = image.height / 2;

    // assign the pixel array for the new image
    half_image.pixels = vec![Pixel::new(); half_image.pixel_count() as usize];

    // loop through each pixel in the new image
    for x in 0..half_image.width {
        for y in 0..half_image.height {
            
            // get the 4 pixels that will be averaged
            let pixel1 = image.get_pixel(x * 2, y * 2);
            let pixel2 = image.get_pixel(x * 2 + 1, y * 2 + 1);
            let pixel3 = image.get_pixel(x * 2 + 1, y * 2);
            let pixel4 = image.get_pixel(x * 2, y * 2 + 1);

            // calculate the average of the 4 pixels
            let avg_r = (pixel1.r + pixel2.r + pixel3.r + pixel4.r) / 4;
            let avg_g = (pixel1.g + pixel2.g + pixel3.g + pixel4.g) / 4;
            let avg_b = (pixel1.b + pixel2.b + pixel3.b + pixel4.b) / 4;

            // create a new pixel
            let avg_pixel = Pixel {
                r: avg_r,
                g: avg_g,
                b: avg_b
            };

            // assign new pixel to new image
            half_image.set_pixel(x, y, &avg_pixel);
        }
    }

    return half_image;
}