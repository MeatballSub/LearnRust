use std::env;
use std::path::Path;

use compress::compress;
use uncompress::uncompress;

mod compress;
mod file_lib;
mod huffman_tree;
mod uncompress;

fn usage(args: &[String])
{
    println!("{} MODE IN OUT", args[0]);
    println!("Huffman compress or uncompress a file");
    println!("MODE is either:");
    println!("  -c: compress");
    println!("  -u: uncompress");
    println!("IN is the input file, it must exist and be a file");
    println!("OUT is the output file, it must NOT already exist, no overwrite functionality");
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args = env::args().collect::<Vec<String>>();

    // should be program + 3 args
    if args.len() != 4
    {
        usage(&args);
        Ok(())
    }
    else
    {
        let mode = &args[1];
        let in_file = Path::new(&args[2]);
        let out_file = Path::new(&args[3]);

        let args_incorrect = (mode != "-c" && mode != "-u") // first arg should be -c(compress) or -u(uncompress)
                           || (!in_file.exists() || !in_file.is_file()) // second arg is input file, it should already exist and be a file
                           || out_file.exists(); // third arg is output file, it should not exist, we don't do overwrites

        if args_incorrect
        {
            usage(&args);
            Ok(())
        }
        // okay safe to proceed
        else if mode == "-c"
        {
            compress(in_file, out_file)?;
            println!("File successfully compressed");
            Ok(())
        }
        else
        {
            uncompress(in_file, out_file)?;
            println!("File successfully uncompressed");
            Ok(())
        }
    }
}
