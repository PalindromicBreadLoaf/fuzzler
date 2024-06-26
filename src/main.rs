use std::fs::File;
use std::io::{self, Read, Write};
use std::mem::size_of;

fn main() -> io::Result<()> {
    let input_file_path = "input.bin";
    let output_file_path = "output.bin";

    // Read the input file
    let mut input_file = File::open(input_file_path)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    // Determine the size of the elements (e.g., 32-bit integers)
    let element_size = size_of::<u16>();

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
            _ => unimplemented!("Only 16, 32, and 64-bit elements are supported."),
        }
    }

    // Write the output file
    let mut output_file = File::create(output_file_path)?;
    output_file.write_all(&buffer)?;

    Ok(())
}
