
use crate::ppm::PPM;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

/* File Operations */
pub fn parse_header(input_file: &File, image: &mut PPM) {
    /*
	parseHeader()

	This function takes in an input file and a PPM object.
	It then reads the header information from the file and assigns it to the PPM object.
	*/

    let mut info_block = 0; // keeps track of which part of the header is being read

    let reader = BufReader::new(input_file); // create a buffered reader to read the file

    // 1. Loop through all lines until all the required data is read (info_block == 3)
    // 2. If the line is a comment, skip it
    // 3. Otherwise, split the line into blocks by spaces
    // 4. For each line, loop over each block until the end of the line, then go to 2
    // 5. Block 1 is the magic number, block 2 is the width, block 3 is the height, block 4 is the max color value
    // 6. Assign the data to the PPM object
    // 7. If the lines run out, exit with an error message

    // loop through each line in the file
    for line in reader.lines() {

        // if all required data has been read, exit the loop
        if info_block >= 4 {
            break;
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
                    image.width = match block.parse::<i32>() {
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
                    image.height = match block.parse::<i32>() {
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
                    image.maxc = match block.parse::<i32>() {
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

    // all data has been read and the image has been modified
}