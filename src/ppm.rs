/*
Contains the struct information for PPM images, as well as the tools for processing them.
*/

/*
The pixel type is a single object that can store rgb data.
r, g, and b are defined as unsigned chars since PPM images
use one byte for r, g, and b, and chars are also one byte.
The unsigned prevents issues with negative numbers when
performing calculations, which can cause messed up image
results.

At a binary level, a single pixel in a PPM file is represented
by three bytes, where the first byte is the r value, the second is the g,
and the third is the b. Because of this, the pixel type cannot be used
for binary operations because it does not contain only 3 bytes. For
file operations, a temporary 3 element array of unsigned char should
be created, where the first element represents the r value, the second
represents the g value, and third represents the b value.
*/

use std::fmt;

#[derive(Copy, Clone)]
pub struct Pixel {
    pub r: u16, // red pixel value
    pub g: u16, // green pixel value
    pub b: u16 // blue pixel value
}

impl Pixel {
    // constructor
    pub fn new() -> Self {
        Pixel {
            r: 0,
            g: 0,
            b: 0
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

impl fmt::Debug for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pixel {{r: {}, g: {}, b: {}}}", self.r, self.g, self.b)
    }
}

/*
The PPM type is the memory representation of a PPM file.
It contains all the information stored in the file header,
such as width, height, maxc, and the magic text, or version,
of the PPM file. It also contains the pixels of the image,
represented by a 1 dimensional array of pixels.
*/
pub struct PPM {
    pub width: u32, // image width
    pub height: u32, // image height
    pub maxc: u32, // max color value (usually 255)
    pub magic: String, // magic number (usually P6)
    pub pixels: Vec<Pixel>, // 1d pixel array
}

impl PPM {
    // constructor
    pub fn new() -> Self {
        PPM {
            width: 0,
            height: 0,
            maxc: 0,
            magic: String::new(),
            pixels: Vec::new()
        }
    }

    /**
    Returns the pixel at the given x and y coordinate.
    */
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<&Pixel> {
        // if x or y are negative, then it will wrap to the other side of the image
        if (x >= self.width) || (y >= self.height) {
            None
        } else {
            Some(&self.pixels[(y * self.width + x) as usize])
        }
    }

    /**
    Sets the pixel at the given x and y coordinate.
    */
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: &Pixel) {
        // if x or y are negative, then it will wrap to the other side of the image
        self.pixels[(y * self.width + x) as usize] = pixel.clone();
    }

    /**
    Returns the total number of pixels in the image.
    */
    pub fn pixel_count(&self) -> u32 {
        self.width * self.height
    }

    /**
    Inverts the pixels in the image.
    */
    pub fn negative(&mut self) {
        // loop over each pixel
        for pixel in &mut self.pixels {
            // invert the pixel
            pixel.r = (self.maxc - pixel.r as u32) as u16;
            pixel.g = (self.maxc - pixel.g as u32) as u16;
            pixel.b = (self.maxc - pixel.b as u32) as u16;
        }
    }

    /**
    Converts the image to grayscale.
    */
    pub fn grayscale(&mut self) {
        // loop over each pixel
        for pixel in &mut self.pixels {
            // calculate the average value of the r, g, and b pixels
            let avg = ((pixel.r as u32 + pixel.g as u32 + pixel.b as u32) / 3) as u16;

            // save the average value to each pixel
            pixel.r = avg;
            pixel.g = avg;
            pixel.b = avg;
        }
    }

}