use std::io::Error;
use std::path::Path;

pub fn uncompress(in_file: &Path, out_file: &Path) -> Result<(), Error>
{
    // open the compressed in_file for reading
    // deserialize the huffman tree from it
    // then decompress the rest of the data and write it to out_file
    todo!()
}
