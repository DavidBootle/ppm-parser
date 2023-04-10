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