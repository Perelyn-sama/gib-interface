use std::fs::{self, File};
use std::io::{self, BufRead, Error, Write};

// fn modify_file(input_file: &str, output_file: &str) -> Result<(), Error> {
//     // Read the content of the input file
//     let content = fs::read_to_string(input_file)?;
//
//     // Process the content - this is where you'd add your own logic
//     let modified_content = content.replace("old_text", "new_text");
//
//     // Write the modified content to the output file
//     let mut file = File::create(output_file)?;
//     file.write_all(modified_content.as_bytes())?;
//
//     Ok(())
// }

// fn copy_line_with_text(input_file: &str, output_file: &str, text: &str) -> Result<(), Error> {
//     // Open the input file
//     let file = File::open(input_file)?;
//     let reader = io::BufReader::new(file);
//
//     // Create or open the output file
//     let mut output = File::create(output_file)?;
//
//     // Iterate over each line in the input file
//     for line in reader.lines() {
//         let line = line?;
//         // Check if the line contains the specified text
//         if line.contains(text) {
//             // Write the line to the output file
//             writeln!(output, "{}", line)?;
//         }
//     }
//
//     Ok(())
// }

// fn main() -> Result<(), Error> {
    // let input_file = "SablierV2NFTDescriptor.sol";
    // let output_file = "ISablierV2NFTDescriptor.sol";
    // let text_to_find = "tokenURI";
    //
    // copy_line_with_text(input_file, output_file, text_to_find)
// }
fn main() {
    File::create("instance.txt");
}
