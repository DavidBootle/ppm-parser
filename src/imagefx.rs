use crate::ppm::PPM;

/** Negative **/
pub fn negative(image: &mut PPM) {
    /*
    This function takes a PPM object as an argument.
	It inverses the pixels contained in the PPM object.
    */

    // loop over each pixel
    for pixel in &mut image.pixels {
        // invert the pixel
        pixel.r = (image.maxc - pixel.r as u32) as u16;
        pixel.g = (image.maxc - pixel.g as u32) as u16;
        pixel.b = (image.maxc - pixel.b as u32) as u16;
    }
}