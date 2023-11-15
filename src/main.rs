use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Error, Read, Write};

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
fn create_template(contract_name: Option<&str>) -> Result<(), io::Error> {
    let instance_buffer = match contract_name {
        Some(str) => File::create(str.to_owned() + ".sol"),
        None => File::create("instance.sol"),
    };
    let f = File::open("outputs/template.sol");
    let mut template_buffer = String::new();
    f?.read_to_string(&mut template_buffer)?;
    instance_buffer?.write_all(template_buffer.as_bytes())?;
    Ok(())
}

pub fn change_interface_name(contract_name: &str) -> Result<(), io::Error> {
    let filename = contract_name.to_owned() + ".sol";
    let content = fs::read_to_string(&filename)?;
    let modified_content = content.replace("Template", contract_name);

    println!("{}", &modified_content);

    let mut file = OpenOptions::new().read(true).write(true).open(&filename)?;
    file.write_all(modified_content.as_bytes())?;
    Ok(())
}

fn extract_line(function_names: Vec<&str>, filename: &str) -> Result<Vec<String>, io::Error> {
    let mut extracts = Vec::new();

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        for function_name in &function_names {
            if line.contains(format!("function {}", function_name).as_str()) {
                extracts.push(line.clone().trim().to_string());
            }
        }
    }

    Ok(extracts)
}

fn main() {
    // let _ = create_template(Some("Contract"));
    // let _ = change_interface_name("Contract");
    let function_names = vec![
        "tokenURI",
        "abbreviateAmount",
        "calculateDurationInDays",
        "calculateStreamedPercentage",
    ];
    let extracts = extract_line(function_names, "SablierV2NFTDescriptor.sol");
    dbg!(extracts.unwrap());
}
// I want to be able to copy the content of the template - done
// change ITemplate to the contract_name - done
// extract line of function name - done
// remove space at the beginning of the extract
// make
