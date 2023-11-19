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

    Ok(format_extract(extracts))
}

fn remove_word(mut sentence: String, word: &str) -> String {
    let index = match sentence.find(word) {
        Some(usize) => usize,
        None => return sentence
    };
    let word_length = word.len();
    for _i in 0..=word_length {
        sentence.remove(index);
    }
    sentence
}

#[test]
fn test_remove_word(){
    let test_string = "function tokenURI(IERC721Metadata sablier, uint256 streamId) external view override returns (string memory uri);".to_string();
    let expected_result = "function tokenURI(IERC721Metadata sablier, uint256 streamId) external view returns (string memory uri);".to_string();
    let result = remove_word(test_string, "override");

    assert_eq!(expected_result, result);
}

fn format_extract(extracts: Vec<String>) -> Vec<String> {
    let mut formatted_extracts = Vec::new();
    for mut extract in extracts {
        if extract.ends_with("{") {
            extract.remove(extract.len() - 1);
        }
        let mut extract = extract.trim().to_string();
        let mut extract = remove_word(extract, "override");
        extract.push(";".parse().unwrap());
        formatted_extracts.push(extract);
    }
    formatted_extracts
}

fn write_to_specific_line(file_path: &str, line_number: usize, new_content: &str) -> std::io::Result<()> {
    // Read the entire file
    let content = fs::read_to_string(file_path)?;

    // Split into lines and collect into a vector
    let mut lines: Vec<&str> = content.lines().collect();

    // Modify the specific line if it exists
    if line_number < lines.len() {
        lines[line_number] = new_content;
    } else {
        // Handle the case where the line number is out of bounds
        // e.g., add error handling or append new lines
    }

    // Rejoin the lines and write back to the file
    let new_content = lines.join("\n");
    fs::write(file_path, new_content)
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
    // dbg!(extracts.unwrap());
    let _ = write_to_specific_line("Contract.sol", 4, extracts.unwrap().join("").as_str());
    // dbg!(format_extract(extracts.unwrap()));
}
// I want to be able to copy the content of the template - done
// change ITemplate to the contract_name - done
// extract line of function name - done
// remove space at the beginning of the extract - done
// fornmat extracts - done
