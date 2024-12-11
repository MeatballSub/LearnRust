use crate::file_lib::bit::Bit;
use std::fs::File;
use std::io::BufReader;
use std::io::Bytes;
use std::io::Error;
use std::io::Read;
use std::path::Path;

pub struct FileBitReader
{
    bytes: Bytes<BufReader<File>>,
    byte: u8,
    mask: u8,
}

impl FileBitReader
{
    pub fn new(path: &Path) -> Self
    {
        let file = File::open(path).unwrap_or_else(|_| panic!("Couldn't open: {:?}", path));
        let buff_reader = BufReader::new(file);
        Self { bytes: buff_reader.bytes(),
               byte: 0,
               mask: 0 }
    }

    pub fn next(&mut self) -> Option<Result<Bit, Error>>
    {
        if self.mask != 0
        {
            Some(Ok(self.extract_bit()))
        }
        else
        {
            match self.next_byte()?
            {
                Ok(_) => Some(Ok(self.extract_bit())),
                Err(e) => Some(Err(e)),
            }
        }
    }

    fn next_byte(&mut self) -> Option<Result<(), Error>>
    {
        const NEW_BYTE_MASK: u8 = 0b10000000;
        let result = self.bytes.next()?;
        match result
        {
            Ok(byte) =>
            {
                self.byte = byte;
                self.mask = NEW_BYTE_MASK;
                Some(Ok(()))
            }
            Err(e) => Some(Err(e)),
        }
    }

    fn extract_bit(&mut self) -> Bit
    {
        let bit = (self.byte & self.mask > 0).into();
        self.mask >>= 1;
        bit
    }
}

#[cfg(test)]
mod tests
{
    use std::io::Write;
    use std::path::PathBuf;

    use super::*;

    struct TestFile
    {
        path: PathBuf,
    }

    impl TestFile
    {
        pub fn create(path: PathBuf) -> Self
        {
            let error_msg = format!("Couldn't create: {:?}", path);
            let mut file = File::create_new(path.clone()).expect(error_msg.as_str());
            file.write(b"Hello World!").expect(error_msg.as_str());
            Self { path }
        }
    }

    impl Drop for TestFile
    {
        fn drop(&mut self) { let _ = std::fs::remove_file(self.path.as_path()); }
    }

    #[test]
    fn test_read()
    {
        let path = Path::new("kmd_FileBitReaderTest.txt");
        let test_file = TestFile::create(path.to_path_buf());
        let test_str = "Hello World!".as_bytes();
        let mut char_index: usize = 0;
        let mut mask: u8 = 0b10000000;

        let mut reader = FileBitReader::new(path);
        while let Some(bit) = reader.next()
        {
            match bit
            {
                Ok(bit) => assert_eq!(bit, (test_str[char_index] & mask > 0).into()),
                Err(_) => assert!(false),
            }

            mask >>= 1;
            if mask == 0
            {
                char_index += 1;
                mask = 0b10000000
            }
        }

        drop(test_file);
    }
}
