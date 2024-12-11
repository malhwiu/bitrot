mod corrupter;

use corrupter::corrupt;

use std::fs;
use std::fs::File;
use std::io::{Read, Write};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long = "verbose")]
    verbose: bool,

    #[arg(short = 'i', long = "input", required = true)]
    file_name: String,

    /// Name of the new folder where all the corrupted files will be placed
    #[arg(short = 'o', long = "output")]
    output: Option<String>,


    #[arg(short = 'n', long = "iter", default_value_t = 1)]
    iterations: usize,

    #[arg(short, long, default_value_t = 0)]
    start: usize,

    #[arg(short, long)]
    end: Option<usize>,


    /// Amount of flipped bits per iteration
    #[arg(short, long)]
    amount: Option<usize>,


    /// 1.0 - All bits will be flipped, 0.0 - none of the bits will be flipped
    #[arg(short, long)]
    ratio: Option<f32>,


    /// Override file extension
    #[arg(long = "override-extension")]
    override_extension: Option<String>
}

fn main() {
    let args = Args::parse();

    let file_name = &args.file_name; 
    let file_name_extention: Vec<&str> = file_name.split(".").collect();

    let mut file_content: Vec<u8> = Vec::new();

    match File::open(file_name) {
        Ok(mut f) => {
            f.read_to_end(&mut file_content).unwrap();
        },
        Err(e) => {
            eprintln!("Failed to open file {}: {}", file_name, e);
            std::process::exit(1);
        }
    }

    let folder_name = match args.output {
        None => {format!("{}_corrupted", file_name)},
        Some(s) => s
    };

    match fs::create_dir(&folder_name) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to create folder {}: {}", folder_name, e);
            std::process::exit(1);
        }
    }

    let start: usize = args.start;
    let end: usize = match args.end {
        Some(v) => v,
        None => {file_content.len()}
    };

    if end < start {
        eprintln!("end cannot be smaller than start");
        std::process::exit(1);
    }

    let iter: usize = args.iterations;
    let amount: usize = if let Some(val) = args.amount {
        val
    } else if let Some(val) = args.ratio {
        (((end - start) as f32) * val) as usize
    } else {
        1
    };

    for i in 0..iter {
        if corrupt(start, end, amount, &mut file_content) {
            let extention = file_name_extention[file_name_extention.len() -1];
            let mut new_file = File::create(format!("{}/{}.{}", folder_name, i.to_string(), extention)).unwrap();
            new_file.write_all(&file_content).unwrap();
            
            if args.verbose {
                println!("[{}/{}]", i + 1, iter);
            }
        }
    }
    

}