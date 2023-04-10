/*
David Bootle
PPM Parser (Rust)
*/

// standard imports
use std::env;
use std::fs::File;
use std::path::Path;
use std::process;
use std::io::{BufReader};

// custom
mod ppm;
mod imageio;
mod imageactions;

use ppm::PPM;
use imageio::{parse_header, read_image_data, write_image};

fn print_help_text() {
    let executable_name = env::args().nth(0).unwrap();

    println!("PPM IMAGE TOOL\n");
    println!("SYNTAX: {} <file> [options]\n", executable_name);
    println!("If run with no options, the tool will output the width and height of the image.\n");

    println!("-h, --help\t\tPrint this help text.");
    println!("-c, --copy\t\tCreate an exact copy of the image.");
    println!("-o [file_path], --output [file_path]\t\tSpecify where to save the image.");
    println!("-n, --negative\t\tConvert the image to a negative.");
    println!("-g, --grayscale\t\tConvert the image to grayscale.");
    println!("-rl, --rotate-left\t\tRotate the image 90 degrees counter-clockwise.");
    println!("-rr, --rotate-right\t\tRotate the image 90 degrees clockwise.");
    println!("-s, --shrink\t\tShrink the image by 2x.");
    println!("-d, --double\t\tDouble the size of the image by turning each pixel into a 2x2 square.");
    println!("-db --double-bilinear\t\tDouble the size of the image using bilinear interpolation.");
    // println!("OPTIONS:");
    // println!("c/C - Create copy");
    // println!("g/G - Convert to grayscale");
    // println!("n/N - Convert to negative");
    // println!("r/R - Rotate clockwise");
    // println!("s/S - Half size (shrink image by 2x)");
    // println!("l/L - Apply LSD-like filter");
    // println!("f/F - Flip image horizontally");
    // println!("ir/IR - Isolate red channel");
    // println!("ig/IG - Isolate blue channel");
    // println!("ib/IB - Isolate green channel");
}
 
fn main() {
 
    // if user only types the executable name and no other arguments, print the help text and exit
    if env::args().len() == 1 {
        print_help_text();
        return;
    }
 
    // since the arguments actually exist, create a vector to store them
    let args: Vec<String> = env::args().collect();

    /* Open Input File */
    let input_file_path = Path::new(&args[1]); // create a new path to represent the input file

    // open the file
    let input_file =  match File::open(&input_file_path) {
        Ok(file) => file, // set input_file to the file if the file exists
        Err(_) => { // otherwise exit with an error message
            eprintln!("Error opening file. Make sure the specified file exists.");
            process::exit(1);
        }
    };

    let mut reader = BufReader::new(input_file); // create a buffered reader to read the file
     
    // create a PPM object which represents the original image in memory, and is what will be read and modified by the program
    let mut image: PPM = PPM::new();

    // process header information and get the end location of the header
    let header_length = parse_header(&mut reader, &mut image);

    // if no options were used, print image header information
    if args.len() == 2 {
        // print image width and height
        println!("Image Dimensions: {} x {}", image.width, image.height);

        // print format subtype
        println!("Format Subtype: {}", image.magic);

        // print bit depth
        let image_bit: &str = match image.maxc {
            255 => "8-bit",
            65535 => "16-bit",
            _ => "Unknown",
        };
        println!("Bit Depth: {}", image_bit);
    }

    // if there are additional arguments, then perform image operations
    if args.len() > 2 {
        
        // verify that the image is of P6 format
        if image.magic != "P6" {
            eprintln!("This image is in {} format. This tool only supports P6 format.", image.magic);
            process::exit(1);
        }

        // read image pixel data
        read_image_data(&mut reader, &mut image, header_length);

        // TEMP
        // create new file to write to
        let filename_no_extension = input_file_path.file_stem().unwrap_or("output".as_ref()).to_str().unwrap_or("output");
        let filename = format!("{}_modified.ppm", filename_no_extension);
        let mut output_file_path = input_file_path.parent().unwrap().join(filename);

        // for each additional argument after the input file, parse the argument and perform the specified operation
        let mut skip_next = false;
        let mut write_image_on_completion = false;

        for i in 2..args.len() {
            if skip_next {
                skip_next = false;
                continue;
            }
            match args[i].as_ref() {

                "-h" | "--help" => {
                    // print the help text
                    print_help_text();
                }

                "-c" | "--copy" => {
                    // create image copy
                    write_image_on_completion = true;
                }

                "-o" | "--output" => {
                    // change the output path name
                    // the argument after this one should be the new output path
                    match args.get(i + 1) {
                        Some(path) => {
                            output_file_path = Path::new(path).to_path_buf();
                            skip_next = true; // skip the next argument since it's the output path
                        }

                        // if there are no arguments after this one, then print an error message
                        None => {
                            eprintln!("No output path specified.");
                        }
                    }
                }

                "-n" | "--negative" => {
                    // convert image to negative
                    image.negative();
                    write_image_on_completion = true;
                }

                "-g" | "--grayscale" => {
                    // convert image to grayscale
                    image.grayscale();
                    write_image_on_completion = true;
                }

                "-rl" | "--rotate-left" => {
                    // // rotate the image counter-clockwise
                    // image = imageactions::rotate_left(image);

                    image = imageactions::rotate_left(image);
                    write_image_on_completion = true;
                }

                "-rr" | "--rotate-right" => {
                    // rotate the image clockwise
                    image = imageactions::rotate_right(image);
                    write_image_on_completion = true;
                }

                "-s" | "--shrink" => {
                    // shrink the image by 2x
                    image = imageactions::half_size(image);
                    write_image_on_completion = true;
                }

                "-d" | "--double" => {
                    // double the size of the image
                    image = imageactions::double_size(image);
                    write_image_on_completion = true;
                }

                "-db" | "--double-bilinear" => {
                    // double the size of the image by using bicubic interpolation
                    image = imageactions::double_bilinear(image);
                    write_image_on_completion = true;
                }

                _ => {
                    println!("Unknown option '{}'. Use option -h to print the help menu.", args[i]);
                }
            }
        }

        if write_image_on_completion {
            write_image(&output_file_path, &image);
            println!("Saved image as '{}'.", output_file_path.canonicalize().expect("Failed to resolve path.").display());
        }
    }
}