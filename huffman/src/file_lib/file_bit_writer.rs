use crate::file_lib::bit::Bit;
use std::fs::File;
use std::io;
use std::io::Error;
use std::io::Write;
use std::path::Path;

const NEW_BYTE_MASK: u8 = 0b10000000;

pub struct FileBitWriter
{
    file: File,
    byte: u8,
    mask: u8,
}

impl FileBitWriter
{
    pub fn new(path: &Path) -> Self
    {
        let file = File::create_new(path).unwrap_or_else(|_| panic!("Couldn't create: {:?}", path));
        Self { file,
               byte: 0,
               mask: NEW_BYTE_MASK }
    }

    pub fn write(&mut self, bit: &Bit) -> Result<(), Error>
    {
        self.set_bit(bit);

        if self.mask == 0
        {
            self.write_byte()?;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Error>
    {
        self.write_byte()?;
        Ok(())
    }

    fn set_bit(&mut self, bit: &Bit)
    {
        if *bit == Bit::One
        {
            self.byte |= self.mask;
        }

        self.mask >>= 1;
    }

    fn write_byte(&mut self) -> io::Result<usize>
    {
        let written = self.file.write(&[self.byte])?;
        self.mask = NEW_BYTE_MASK;
        self.byte = 0;

        Ok(written)
    }
}

impl Drop for FileBitWriter
{
    fn drop(&mut self)
    {
        if self.mask != NEW_BYTE_MASK
        {
            self.write_byte().expect("failed writing byte in drop");
        }
    }
}

#[cfg(test)]
mod tests
{
    use std::path::PathBuf;

    use super::*;

    struct TestFile
    {
        path: PathBuf,
    }

    impl TestFile
    {
        pub fn create(path: PathBuf) -> Self { Self { path } }
    }

    impl Drop for TestFile
    {
        fn drop(&mut self) { let _ = std::fs::remove_file(self.path.as_path()); }
    }

    fn write_char(writer: &mut FileBitWriter, c: u8)
    {
        let mut mask = 0x80;
        while mask != 0
        {
            let bit = (c & mask > 0).into();
            writer.write(&bit).expect("failed writing bit");
            mask >>= 1;
        }
    }

    #[test]
    fn test_write()
    {
        let path = Path::new("kmd_FileBitWriterTest.txt");
        let test_file = TestFile::create(path.to_path_buf());
        let test_str = "Hello World!";

        let mut writer = FileBitWriter::new(path);

        for c in test_str.as_bytes()
        {
            write_char(&mut writer, *c);
        }

        let read_str = std::fs::read_to_string(path).expect("failed reading file");

        assert_eq!(read_str, test_str);

        drop(test_file);
    }

    #[test]
    fn test_partial_byte_write()
    {
        let path = Path::new("kmd_FileBitWriterTestPartial.txt");
        let test_file = TestFile::create(path.to_path_buf());
        let test_str = "Hello World!";

        let mut writer = FileBitWriter::new(path);

        for c in test_str.as_bytes()
        {
            write_char(&mut writer, *c);
        }

        let mut mask = 0x80;
        let c = b'5';
        while mask != 0x08
        {
            let bit = (c & mask > 0).into();
            writer.write(&bit).expect("failed writing bit");
            mask >>= 1;
        }

        // flush partial byte
        writer.flush().expect("error flushing partial byte");

        let read_str = std::fs::read_to_string(path).expect("failed reading file");

        // high nibble was written so instead of '5' should have wrote '0'
        assert_eq!(read_str, "Hello World!0");

        drop(test_file);
    }
}
