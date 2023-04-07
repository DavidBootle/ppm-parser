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

pub struct Pixel {
    pub r: u8, // red pixel value
    pub g: u8, // green pixel value
    pub b: u8 // blue pixel value
}

/*
The PPM type is the memory representation of a PPM file.
It contains all the information stored in the file header,
such as width, height, maxc, and the magic text, or version,
of the PPM file. It also contains the pixels of the image,
represented by a 1 dimensional array of pixels.
*/
pub struct PPM {
    pub width: i32, // image width
    pub height: i32, // image height
    pub maxc: i32, // max color value (usually 255)
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
}