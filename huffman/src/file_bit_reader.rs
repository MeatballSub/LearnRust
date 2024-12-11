use crate::bit::Bit;
use std::fs::File;
use std::io::BufReader;
use std::io::Bytes;
use std::io::Error;
use std::io::Read;
use std::path::Path;

const NEW_BYTE_MASK: u8 = 0b10000000;

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
        let error_msg = format!("Couldn't open: {:?}", path);
        let file = File::open(path).expect(error_msg.as_str());
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
        let result = self.bytes.next()?;
        match result
        {
            Ok(byte) =>
            {
                self.byte = byte;
                self.mask = NEW_BYTE_MASK;
                Some(Ok(()))
            }
            Err(e) => return Some(Err(e)),
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

    fn test_char(reader: &mut FileBitReader, c: u8)
    {
        let mut mask = 0x80;
        while mask != 0
        {
            let actual = reader.next().unwrap().unwrap();
            let expected = (c & mask > 0).into();
            assert_eq!(actual, expected,
                       "{:?} != {:?}, with mask {}",
                       actual, expected, mask);
            mask >>= 1;
        }
    }

    #[test]
    fn test_read()
    {
        let path = Path::new("kmd_FileBitReaderTest.txt");
        let test_file = TestFile::create(path.to_path_buf());

        let mut reader = FileBitReader::new(path);
        test_char(&mut reader, b'H');
        test_char(&mut reader, b'e');
        test_char(&mut reader, b'l');
        test_char(&mut reader, b'l');
        test_char(&mut reader, b'o');
        test_char(&mut reader, b' ');
        test_char(&mut reader, b'W');
        test_char(&mut reader, b'o');
        test_char(&mut reader, b'r');
        test_char(&mut reader, b'l');
        test_char(&mut reader, b'd');
        test_char(&mut reader, b'!');

        drop(test_file);
    }
}
