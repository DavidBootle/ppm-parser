
use crate::ppm::{PPM, Pixel};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, BufWriter, Write};
use std::path::Path;
use std::process;

/* File Operations */
pub fn parse_header(reader: &mut BufReader<File>, image: &mut PPM) -> usize {
    /*
	parseHeader()

	This function takes in an input file and a PPM object.
	It then reads the header information from the file and assigns it to the PPM object.
	*/

    let mut info_block = 0; // keeps track of which part of the header is being read

    // 1. Loop through all lines until all the required data is read (info_block == 3)
    // 2. If the line is a comment, skip it
    // 3. Otherwise, split the line into blocks by spaces
    // 4. For each line, loop over each block until the end of the line, then go to 2
    // 5. Block 1 is the magic number, block 2 is the width, block 3 is the height, block 4 is the max color value
    // 6. Assign the data to the PPM object
    // 7. If the lines run out, exit with an error message

    // loop through each line in the file
    for (index, line) in reader.lines().enumerate() {

        // if all required data has been read, exit the loop
        if info_block >= 4 {
            return index; // return the line number where the header ends
        }
        
        // get the current line, or exit with error if lines run out
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                eprintln!("End of file reached before all header information was read.");
                process::exit(1);
            }
        };

        // if the line is a comment, skip it
        if line.starts_with("#") {
            continue;
        }

        // split the line into blocks by spaces
        let blocks: Vec<&str> = line.split(' ').collect();

        // loop through each block in the line
        for block in blocks {

            // save the current block as a property of the PPM object depending on which block it is
            match info_block {
                0 => {
                    // Block 1: Magic Number
                    image.magic = block.to_string();
                    info_block += 1;
                }

                1 => {
                    // Block 2: Width
                    image.width = match block.parse::<u32>() {
                        Ok(num) => num,
                        Err(_) => {
                            eprintln!("Header block for width failed to parse. Possibly invalid file type.");
                            process::exit(1);
                        }
                    };
                    info_block += 1;
                }

                2 => {
                    // Block 3: Height
                    image.height = match block.parse::<u32>() {
                        Ok(num) => num,
                        Err(_) => {
                            eprintln!("Header block for height failed to parse. Possibly invalid file type.");
                            process::exit(1);
                        }
                    };
                    info_block += 1;
                }

                3 => {
                    // Block 4: Max Color
                    image.maxc = match block.parse::<u32>() {
                        Ok(maxc) => maxc,
                        Err(_) => {
                            eprintln!("Header block for max color failed to parse. Possibly invalid file type.");
                            process::exit(1);
                        }
                    };
                    info_block += 1;
                }

                _ => {
                    // info_block is none of the above, which means we've read all required blocks
                    break;
                }
            }
        }
    }

    return 0;

    // all data has been read and the image has been modified
    // in addition, the reader is now at the start of the pixel data
}


pub fn read_image_data(reader: &mut BufReader<File>, image: &mut PPM, header_length: usize) {
    /*
	readImageData()

	This function takes reader and a PPM object as arguments.
	It reads the binary data from the file and stores it as pixels in the PPM object.
	This function requires that parseHeader was run previously, or else the file stream
	cursor will in the wrong place.
	*/

    // Reset the seeker and move it to the start of the pixel data
    reader.seek(SeekFrom::Start(0)).expect("Failed to seek to start of file.");
    let mut tmp_str = String::new();
    for _ in 0..header_length {
        reader.read_line(&mut tmp_str).expect("Failed to skip line of the header.");
    }

    // initialize the image pixel data
    image.pixels = Vec::with_capacity(image.pixel_count() as usize);

    // loop through each pixel in the image
    for _ in 0..image.pixel_count() {
        // Process 8 bit data
        match image.maxc {
            255 => {
                // Process 8 bit color data
                let mut buffer = [0u8; 3]; // create a temporary buffer to store the pixel data
                match reader.read_exact(&mut buffer) { // read the data into the buffer
                    Ok(_) => {
                        // store the pixel info in the PPM object
                        let pixel = Pixel {
                            r: buffer[0] as u16,
                            g: buffer[1] as u16,
                            b: buffer[2] as u16,
                        };
                        image.pixels.push(pixel);
                    }
                    Err(error) => {
                        eprintln!("{:?}", error);
                        eprintln!("End of file reached before all pixel data was read. File may be corrupted.");
                        process::exit(1);
                    }
                };
            }

            65535 => {
                // Process 16 bit color data
                let mut buffer = [0u8; 6]; // create a temporary buffer to store the pixel data
                match reader.read_exact(&mut buffer) { // read the data into the buffer
                    Ok(_) => {
                        // store the pixel info in the PPM object
                        let pixel = Pixel {
                            r: u16::from_be_bytes([buffer[0], buffer[1]]),
                            g: u16::from_be_bytes([buffer[2], buffer[3]]),
                            b: u16::from_be_bytes([buffer[4], buffer[5]])
                        };
                        image.pixels.push(pixel);
                    }
                    Err(_) => {
                        eprintln!("End of file reached before all pixel data was read. File may be corrupted.");
                        process::exit(1);
                    }
                };
            }

            _ => {
                // Invalid bit depth
                eprintln!("Cannot parse pixel data for image with max color value of {}.", image.maxc);
                process::exit(1);
            }
        };
    }
}

pub fn write_image(output_file_path: &Path, image: &PPM) {
    /*
	writeImageToFile()

	This function takes a filestream pointer and a PPM object pointer as arguments.
	It writes a new P6 PPM file to the filestream pointer using the data in the PPM object.
	The filestream pointer must reference an opened filestream that is in write mode.
	*/

    // open file for writing
    let out_file = match File::create(output_file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error writing to output file.");
            process::exit(1);
        }
    };

    // Create a new filestream writer
    let mut writer = BufWriter::new(out_file);

    // Create the new header text
    let header = format!("{}\n# Modified with David Bootle's PPM Image Tool\n{} {}\n{}\n", image.magic, image.width, image.height, image.maxc);

    // Write the header to the file
    writer.write(header.as_bytes()).expect("Failed to write header to file.");

    // Write the pixel data to the file
    for pixel in &image.pixels {
        match image.maxc {
            255 => {
                // Write 8 bit color data
                let mut buffer = [0u8; 3];
                buffer[0] = pixel.r as u8;
                buffer[1] = pixel.g as u8;
                buffer[2] = pixel.b as u8;
                writer.write(&buffer).expect("Failed to write pixel data to file.");
            }

            65535 => {
                // Write 16 bit color data
                let mut buffer = [0u8; 6];
                buffer[0..2].copy_from_slice(&pixel.r.to_be_bytes());
                buffer[2..4].copy_from_slice(&pixel.g.to_be_bytes());
                buffer[4..6].copy_from_slice(&pixel.b.to_be_bytes());
                writer.write(&buffer).expect("Failed to write pixel data to file.");
            }

            _ => {
                // Invalid bit depth
                eprintln!("Cannot write pixel data for image with max color value of {}.", image.maxc);
                process::exit(1);
            }
        }
    }
}