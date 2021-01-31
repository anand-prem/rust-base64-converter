use std::env;
use std::fs;
use std::fs::File;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::Write;

use serde_json;
use base64::{decode};

fn is_string_starts_with_double_dash(input: &str) -> bool {
    return input.starts_with("--");
}

fn get_options_from_cli() -> HashMap<String, String> {
    const CMD_ARGS: [&str; 3] = ["file", "path", "output"];
    let mut program_options = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let args_count = args.len();
    let mut i = 1;
    while i < args_count {
        if is_string_starts_with_double_dash(&args[i]) {
            if (i+1) >= args_count {
                break;
            }
            let arg_key = args[i].strip_prefix("--").unwrap().to_string();
            let arg_value = args[i+1].to_string();
            if !is_string_starts_with_double_dash(&arg_value) && CMD_ARGS.contains(&&arg_key[..]) {
                program_options.insert(arg_key, arg_value);
            }
            i += 1;
        }
        i += 1;
    }
    for index in 0..CMD_ARGS.len() {
        if !program_options.contains_key(CMD_ARGS[index]) {
            panic!("Missing option `{}`", CMD_ARGS[index]);
        }
    }
    return program_options;
}

fn get_file_content(file_name: &String) -> serde_json::Value {
    let contents = match fs::read_to_string(file_name) {
        Ok(file) => file,
        Err(error) => panic!("FileReadFailed: {}", error),
    };
    let json_contents: serde_json::Value = match serde_json::from_str(&contents[..]) {
        Ok(json_data) => json_data,
        Err(error) => panic!("JSON parse failed: {}", error),
    };
    return json_contents;
}

fn get_path_value_from_data(path: &String, data: serde_json::Value) -> serde_json::Value {
    let path_array: Vec<&str> = path.split(".").collect();
    let mut value_at_path: serde_json::Value = data;
    for key in path_array {
        match key.parse::<usize>() {
            Ok(number_key) => value_at_path = value_at_path[number_key].borrow().clone(),
            Err(_) => value_at_path = value_at_path[key].borrow().clone(),
        };
    }
    return value_at_path;
}

fn write_data_to_file(output_path: &String, data: &str) {
    let mut file = match File::create(output_path) {
        Ok(f) => f,
        Err(error) => panic!("Error creating output file: {}", error),
    };
    match file.write(&decode(data).unwrap()[..]) {
        Ok(some_data) => println!("write_data_to_file {}", some_data),
        Err(error) => panic!("Error while writing to output: {}", error),
    }
}

fn main() {
    let options = get_options_from_cli();
    println!("{:?}", options);
    let file_contents = get_file_content(options.get("file").unwrap());
    let base64_value = get_path_value_from_data(options.get("path").unwrap(), file_contents);
    write_data_to_file(options.get("output").unwrap(), base64_value.as_str().unwrap())
}
