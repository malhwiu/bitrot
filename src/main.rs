mod corrupter;

use corrupter::{CorruptRatio, corrupt};

use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file name is provided
    if args.len() < 6 {
        eprintln!("Usage: {} <filename> <iterations> <amount> <start> <end>", args[0]);
        std::process::exit(1);
    }

    // Read the file name from the arguments
    let file_name = &args[1]; 

    let file_name_extention: Vec<&str> = file_name.split(".").collect();

    let mut file_content: Vec<u8> = Vec::new();

    // Open the file to ensure it exists and is accessible
    match fs::File::open(file_name) {
        Ok(mut f) => {
            f.read_to_end(&mut file_content).unwrap();
        },
        Err(e) => {
            eprintln!("Failed to open file {}: {}", file_name, e);
            std::process::exit(1);
        }
    }

    // Create a folder named "<filename>_corrupted"
    let folder_name = format!("{}_corrupted", file_name);
    match fs::create_dir(&folder_name) {
        Ok(_) => println!("Successfully created folder: {}", folder_name),
        Err(e) => {
            eprintln!("Failed to create folder {}: {}", folder_name, e);
            std::process::exit(1);
        }
    }

    let iter: usize = args[2].parse().unwrap();
    let amount: usize = args[3].parse().unwrap();

    let start: usize = args[4].parse().unwrap();
    let mut end: usize = args[5].parse().unwrap();

    if end == 0 {
        end = file_content.len();
    }


    for i in 0..iter {
        if corrupt(start, end, &CorruptRatio::Amount(amount), &mut file_content) {
            let extention = file_name_extention[file_name_extention.len() -1];
            let mut new_file = File::create(format!("{}/{}.{}", folder_name, i.to_string(), extention)).unwrap();
            new_file.write_all(&file_content).unwrap();
        }
    }
    

}
