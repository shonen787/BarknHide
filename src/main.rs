use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_woff: String,
    #[arg(short, long)]
    output_woff: String,
    #[arg(short, long)]
    bin_file_path: String,
}

fn main() {
    let args = Args::parse();
    let woff2_file_path = args.input_woff.as_str();
    let bin_file_path = args.bin_file_path.as_str();
    let output_file_path = args.output_woff.as_str();

    let mut woff2_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(woff2_file_path)
        .expect("Failed to open WOFF2 file.");

    let bin_file = File::open(bin_file_path).expect("Failed to open binary file.");

    let mut woff2_data = Vec::new();
    woff2_file
        .read_to_end(&mut woff2_data)
        .expect("Failed to read WOFF2 file.");

    let bin_data: Vec<u8> = bin_file.bytes().filter_map(|byte| byte.ok()).collect();

    let original_length =
        u32::from_be_bytes([woff2_data[8], woff2_data[9], woff2_data[10], woff2_data[11]]);

    woff2_data.extend_from_slice(&bin_data);

    let new_length = (original_length + bin_data.len() as u32).to_be_bytes();

    woff2_data[8] = new_length[0];
    woff2_data[9] = new_length[1];
    woff2_data[10] = new_length[2];
    woff2_data[11] = new_length[3];

    let private_offset_bytes = original_length.to_be_bytes();
    woff2_data[36] = private_offset_bytes[0];
    woff2_data[37] = private_offset_bytes[1];
    woff2_data[38] = private_offset_bytes[2];
    woff2_data[39] = private_offset_bytes[3];

    let private_data_length = bin_data.len() as u32;
    let private_data_length_bytes = private_data_length.to_be_bytes();

    woff2_data[40] = private_data_length_bytes[0];
    woff2_data[41] = private_data_length_bytes[1];
    woff2_data[42] = private_data_length_bytes[2];
    woff2_data[43] = private_data_length_bytes[3];

    let mut output_file = File::create(output_file_path).expect("Failed to create output file.");
    output_file
        .write_all(&woff2_data)
        .expect("Failed to write to output file.");
}
