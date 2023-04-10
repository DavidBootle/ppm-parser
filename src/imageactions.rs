use crate::ppm::{PPM, Pixel};
use rayon::prelude::*;

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

/**
Halfs the size of the image by averaging the pixels.
 */
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
            let pixel1 = image.get_pixel(x * 2, y * 2).unwrap();
            let pixel2 = image.get_pixel(x * 2 + 1, y * 2 + 1).unwrap();
            let pixel3 = image.get_pixel(x * 2 + 1, y * 2).unwrap();
            let pixel4 = image.get_pixel(x * 2, y * 2 + 1).unwrap();

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


/**
Doubles the size of an image by copying each pixel into a 2x2 square
*/
pub fn double_size(image: PPM) -> PPM {
    let mut double_image = PPM::new();

    // copy header info
    double_image.magic = image.magic.clone();
    double_image.maxc = image.maxc;
    double_image.width = image.width * 2;
    double_image.height = image.height * 2;

    // assign the pixel array for the new image
    double_image.pixels = vec![Pixel::new(); double_image.pixel_count() as usize];

    // loop through each pixel in the old image
    for x in 0..image.width {
        for y in 0..image.height {
            
            // get the pixel that will be doubled
            let pixel = image.get_pixel(x, y).unwrap();

            // assign new pixel to new image
            double_image.set_pixel(x * 2, y * 2, &pixel);
            double_image.set_pixel(x * 2 + 1, y * 2, &pixel);
            double_image.set_pixel(x * 2, y * 2 + 1, &pixel);
            double_image.set_pixel(x * 2 + 1, y * 2 + 1, &pixel);
        }
    }

    return double_image;
}

/**
Calculates the value of an "in-between" pixel of an image using bilinear interpolation.
Used in the double_bilinear function.
*/
fn bilinear_interpolation(image: &PPM, x: f32, y: f32) -> Pixel {
    // get the x and y values of the pixel on the original image

    // get the x and y values of the 4 pixels that will be used for interpolation
    let x1 = x.floor();
    let x2 = x.floor() + 1.0;
    let y1 = y.floor();
    let y2 = y.floor() + 1.0;

    // get the 4 pixels that will be used for interpolation
    let q11 = image.get_pixel(x1 as u32, y1 as u32).unwrap(); // guaranteed to be in the image

    // for the rest, if the pixel is in the image, then return the pixel
    // otherwise, return q11, since the referenced pixel is out of bounds and doesn't exist
    let q21 = match image.get_pixel(x2 as u32, y1 as u32) {
        Some(pixel) => pixel,
        None => q11
    };
    let q12 = match image.get_pixel(x1 as u32, y2 as u32) {
        Some(pixel) => pixel,
        None => q11
    };
    let q22 = match image.get_pixel(x2 as u32, y2 as u32) {
        Some(pixel) => pixel,
        None => q11
    };

    // red channel
    let r1_r = q11.r as f32 * (x2 - x) / (x2 - x1) + q21.r as f32 * (x - x1) / (x2 - x1);
    let r2_r: f32 = q12.r as f32 * (x2 - x) / (x2 - x1) + q22.r as f32 * (x - x1) / (x2 - x1);
    let p_r = r1_r * (y2 - y) / (y2 - y1) + r2_r * (y - y1) / (y2 - y1);
    
    // green channel
    let r1_g = q11.g as f32 * (x2 - x) / (x2 - x1) + q21.g as f32 * (x - x1) / (x2 - x1);
    let r2_g: f32 = q12.g as f32 * (x2 - x) / (x2 - x1) + q22.g as f32 * (x - x1) / (x2 - x1);
    let p_g = r1_g * (y2 - y) / (y2 - y1) + r2_g * (y - y1) / (y2 - y1);

    // blue channel
    let r1_b = q11.b as f32 * (x2 - x) / (x2 - x1) + q21.b as f32 * (x - x1) / (x2 - x1);
    let r2_b: f32 = q12.b as f32 * (x2 - x) / (x2 - x1) + q22.b as f32 * (x - x1) / (x2 - x1);
    let p_b = r1_b * (y2 - y) / (y2 - y1) + r2_b * (y - y1) / (y2 - y1);
    
    // create new interpolated pixel
    let interpolated_pixel = Pixel {
        r: p_r as u16,
        g: p_g as u16,
        b: p_b as u16
    };

    interpolated_pixel

}

/**
Doubles the size of an image by using bilinear interpolation.
*/
pub fn double_bilinear(image: PPM) -> PPM {
    let mut double_image = PPM::new();

    // copy header info
    double_image.magic = image.magic.clone();
    double_image.maxc = image.maxc;
    double_image.width = image.width * 2;
    double_image.height = image.height * 2;

    // assign the pixel array for the new image
    double_image.pixels = vec![Pixel::new(); double_image.pixel_count() as usize];

    // loop through each pixel in the new image
    double_image.pixels.par_iter_mut().enumerate().for_each(|(index, pixel)| {
        let x = (index as u32) % double_image.width;
        let y = (index as u32) / double_image.width;

        let new_pixel = bilinear_interpolation(&image, x as f32 / 2.0, y as f32 / 2.0);
        *pixel = new_pixel;
    });

    return double_image;
}

/**
Flips an image in the horizontal direction.
*/
pub fn flip_horizontal(image: PPM) -> PPM {
    let mut flipped_image = PPM::new();

    // copy header info
    flipped_image.magic = image.magic.clone();
    flipped_image.maxc = image.maxc;
    flipped_image.width = image.width;
    flipped_image.height = image.height;

    // assign the pixel array for the new image
    flipped_image.pixels = vec![Pixel::new(); flipped_image.pixel_count() as usize];

    // loop through each pixel in the new image
    for x in 0..flipped_image.width {
        for y in 0..flipped_image.height {
            let pixel = image.get_pixel(flipped_image.width - x -1, y).unwrap(); // guaranteed to be in the image
            flipped_image.set_pixel(x, y, &pixel);
        }
    }

    return flipped_image;
}

/**
Flips an image in the vertical direction.
*/
pub fn flip_vertical(image: PPM) -> PPM {
    let mut flipped_image = PPM::new();

    // copy header info
    flipped_image.magic = image.magic.clone();
    flipped_image.maxc = image.maxc;
    flipped_image.width = image.width;
    flipped_image.height = image.height;

    // assign the pixel array for the new image
    flipped_image.pixels = vec![Pixel::new(); flipped_image.pixel_count() as usize];

    // loop through each pixel in the new image
    for x in 0..flipped_image.width {
        for y in 0..flipped_image.height {
            let pixel = image.get_pixel(x, flipped_image.height - y - 1).unwrap(); // guaranteed to be in the image
            flipped_image.set_pixel(x, y, &pixel);
        }
    }

    return flipped_image;
}