use std::fs::File;
use std::io::{self, Read, Write};
use std::io::stdin;

fn main() -> io::Result<()> {
    let input_file_path = "input.bin";
    let output_file_path = "output.bin";

    // Prompt the user for the element size
    println!("Enter the element size (in bytes):");
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let element_size: usize = input.trim().parse().expect("Invalid input");

    // Read the input file
    let mut input_file = File::open(input_file_path)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    // Reverse endianness of each element
    for chunk in buffer.chunks_exact_mut(element_size) {
        match element_size {
            2 => {
                let mut value = u16::from_le_bytes(chunk.try_into().unwrap());
                value = value.swap_bytes();
                chunk.copy_from_slice(&value.to_le_bytes());
            }
            4 => {
                let mut value = u32::from_le_bytes(chunk.try_into().unwrap());
                value = value.swap_bytes();
                chunk.copy_from_slice(&value.to_le_bytes());
            }
            8 => {
                let mut value = u64::from_le_bytes(chunk.try_into().unwrap());
                value = value.swap_bytes();
                chunk.copy_from_slice(&value.to_le_bytes());
            }
            _ => panic!("Unsupported element size."),
        }
    }

    // Write the output file
    let mut output_file = File::create(output_file_path)?;
    output_file.write_all(&buffer)?;

    Ok(())
}
