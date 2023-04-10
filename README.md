# ppm-parser

A rust based version of a project I made for C lab. This was my first rust project. Although the PPM file format is archaic and an absolute mess, it is fun to mess with due to the lack of compression. It is very straightfoward to work with, and so a PPM image parser makes a great first rust project. Through this project, I learned the following in Rust:
- How to create and use structs and implement traits
- How to work with and build multi-file projects with cargo
- How to read, write, and otherwise manage files
- How to implement multithreaded parallel processing
- How to use the unique match system

## How to Build
Clone the project and run `cargo build --release` to generate the binary for your system. The binary can be found as `target/release/ppm-parser`. It's completely standalone.

## How to Use
```
SYNTAX: ppmparser <file> [options]

If run with no options, the tool will output the width and height of the image.

-h, --help              Print this help text.
-c, --copy              Create an exact copy of the image.
-o [file_path], --output [file_path]            Specify where to save the image.
-t --time               Print the time it took to run the program.

Image Effects
-n, --negative          Convert the image to a negative.
-g, --grayscale         Convert the image to grayscale.
-rl, --rotate-left              Rotate the image 90 degrees counter-clockwise.
-rr, --rotate-right             Rotate the image 90 degrees clockwise.
-s, --shrink            Shrink the image by 2x.
-d, --double            Double the size of the image by turning each pixel into a 2x2 square.
-db --double-bilinear           Double the size of the image using bilinear interpolation.
-fh --flip-horizontal           Flip the image horizontally.
-fv --flip-vertical             Flip the image vertically.
-ir --isolate-red               Isolate the red channel of the image.
-ig --isolate-green             Isolate the green channel of the image.
-ib --isolate-blue              Isolate the blue channel of the image.
-Dr --delete-red                Delete the red channel of the image.
-Dg --delete-green              Delete the green channel of the image.
-Db --delete-blue               Delete the blue channel of the image.
```

All of the image effects can be stacked. For example `ppmparser myimage.ppm -n -n` will result in normal image, since you took the negative twice.

## Conclusions on Rust?
Rust is a fantastic language. It's got the speed of C with the convience of Python, and the memory safety features eliminate most of the stress of low-level programming. The Option and Result types are very unique concepts, and now that I've used them, it's strange that no other programming language has seemed to pick them up. Combined with the match statement, it makes error handling insanely easy, and I don't have to worry about weird edge cases anymore. Despite being a low level lanugage, Rust is filled to the brim with useful convience functions, while still allowing you to get that incredibly low level control. Finally!

Consider me a member of the Rustaceans. I've been converted. But seriously, if you haven't tried rust, just try it. You'd be suprised at just how flexible it is.
