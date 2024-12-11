use std::io::Error;
use std::path::Path;

pub fn compress(in_file: &Path, out_file: &Path) -> Result<(), Error>
{
    // Build the frequency table
    // Create a huffman tree from the frequency table
    // Serialize the huffman tree to the out_file
    // Open the in_file process it and write out the compressed version to the out_file
    // Write out an eof pattern
    todo!()
}
