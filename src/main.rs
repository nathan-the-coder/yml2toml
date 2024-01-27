use std::env;
use std::fs;
use std::path::Path;
use serde_yaml::Value;
use toml::Value as TomlValue;

fn display_help() {
    println!("Usage: {} input_file.yml", env::args().next().unwrap());
    println!("Converts YAML file to TOML format.");
    std::process::exit(1);
}

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || args[1] == "-h" || args[1] == "--help" {
        display_help();
    }

    // Read input file
    let input_file = &args[1];
    let input_content = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(_) => {
            panic!("Error: Unable to read input file '{}'", input_file);
        }
    };

    // Parse YAML
    let yaml_value: Value = match serde_yaml::from_str(&input_content) {
        Ok(value) => value,
        Err(_) => {
            panic!("Error: Failed to parse YAML from input file '{}'", input_file);
        }
    };

    // Convert to TOML 
    let toml_value: TomlValue = match toml::ser::to_string(&yaml_value) {
        Ok(value) => toml::Value::String(value),
        Err(_) => {
            panic!("Error: Failed to convert YAML to TOML.");
        }
    };

    // Write TOML to output file
    let output_file = format!("{}.toml", Path::new(input_file).file_stem().unwrap().to_str().unwrap());
    match fs::write(&output_file, toml::ser::to_string_pretty(&toml_value).expect("Value is not string").replace("'", "")) {
        Ok(_) => println!("Conversion successful. Output saved to {}", output_file),
        Err(_) => {
            eprintln!("Error: Failed to write TOML to output file '{}'", output_file);
        }
    }
}
