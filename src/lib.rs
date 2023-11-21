use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Read, Write};
use std::process;
// use clap::error::ContextValue::String;

pub fn create_template(contract_name: Option<&str>) -> Result<(), io::Error> {
    let instance_buffer = match contract_name {
        Some(str) => File::create(format!("I{str}.sol")),
        None => File::create("instance.sol"),
    };
    let mut f = File::open("template.sol")?;
    let mut template_buffer = String::new();
    f.read_to_string(&mut template_buffer)?;
    instance_buffer?.write_all(template_buffer.as_bytes())?;
    Ok(())
}

pub fn change_interface_name(contract_name: &str) -> Result<(), io::Error> {
    let filename = format!("I{contract_name}.sol");
    let content = fs::read_to_string(&filename)?;
    let modified_content = content.replace("Template", contract_name);
    let mut file = OpenOptions::new().read(true).write(true).open(&filename)?;
    file.write_all(modified_content.as_bytes())?;
    Ok(())
}

pub fn extract_line(function_names: Vec<String>, filename: &str) -> Result<Vec<String>, io::Error> {
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

pub fn remove_word(mut sentence: String, word: &str) -> String {
    let index = match sentence.find(word) {
        Some(usize) => usize,
        None => return sentence,
    };
    let word_length = word.len();
    for _i in 0..=word_length {
        sentence.remove(index);
    }
    sentence
}

pub fn format_extract(extracts: Vec<String>) -> Vec<String> {
    let mut formatted_extracts = Vec::new();
    for mut extract in extracts {
        if extract.ends_with("{") {
            extract.remove(extract.len() - 1);
        }
        let extract = extract.trim().to_string();
        let mut extract = remove_word(extract, "override");
        extract.push(";".parse().unwrap());
        formatted_extracts.push(extract);
    }
    formatted_extracts
}

pub fn write_to_specific_line(
    file_path: &str,
    line_number: usize,
    new_content: &str,
) -> std::io::Result<()> {
    let content = fs::read_to_string(file_path)?;

    let mut lines: Vec<&str> = content.lines().collect();

    if line_number < lines.len() {
        lines[line_number] = new_content;
    } else {
        // TODO
        // Handle the case where the line number is out of bounds
        // e.g., add error handling or append new lines
    }

    let new_content = lines.join("\n");
    fs::write(file_path, new_content)
}

pub fn run(contract_file: String, function_names: Vec<String>) {
    let interface_name: String = "SablierV2NFTDescriptor.sol".split(".").take(1).collect();
    let interface_name = interface_name.as_str();

    let _ = create_template(Some(interface_name));
    let _ = change_interface_name(interface_name);

    let extracts = extract_line(function_names, contract_file.as_str());

    let _ = write_to_specific_line(
        format!("I{interface_name}.sol").as_str(),
        4,
        extracts.unwrap().join("").as_str(),
    );

    let _ = process::Command::new("forge")
        .args(["fmt", "contract.sol"])
        .output();
}

#[test]
fn test_remove_word() {
    let test_string = "function tokenURI(IERC721Metadata sablier, uint256 streamId) external view override returns (string memory uri);".to_string();
    let expected_result = "function tokenURI(IERC721Metadata sablier, uint256 streamId) external view returns (string memory uri);".to_string();
    let result = remove_word(test_string, "override");

    assert_eq!(expected_result, result);
}
